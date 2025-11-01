<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { browser } from '$app/environment';
  import 'leaflet/dist/leaflet.css';

  import { PUBLIC_BACKEND_URL } from '$env/static/public';
  import { auth } from '$lib/stores/auth';
  import { get } from 'svelte/store';

  const demoFindings: Finding[] = [
    { id: '1', lat: 47.5596, lng: 7.5886, weight: 40, category: 'Beverage Can', material: 'Aluminium', brand: 'Smirnoff' },
    { id: '2', lat: 47.5591, lng: 7.5859, weight: 20, category: 'Snack Wrapper', material: 'Plastic' },
    { id: '3', lat: 47.5584, lng: 7.5920, weight: 20, category: 'Plastic Bottle', material: 'Plastic', brand: 'Fanta' }
  ];

  // --- leaflet refs ---
  let mapDiv: HTMLDivElement;
  let map: any;
  let markersLayer: any;
  let L: any;
  let delitterIcon: any;

  // user location layers
  let userMarker: any = null;
  let userAccuracy: any = null;
  let geoWatchId: number | null = null;

  // --- helpers ---
  function escapeHtml(s: string) {
    return s.replace(/[&<>"']/g, (c) => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]!));
  }

  function popupHtml(f: Finding) {
    const brand = f.brand ? `<span class="badge badge-outline">${escapeHtml(f.brand)}</span>` : '';
    const material = `<span class="badge badge-success badge-outline">${escapeHtml(f.material)}</span>`;
    const weight = f.weight != null ? `<span class="text-xs text-base-content/60">${f.weight} g</span>` : '';
    return `
      <div class="card bg-base-100">
        <div class="card-body p-4">
          <h3 class="card-title text-base">${escapeHtml(f.category)}</h3>
          <div class="flex flex-wrap items-center gap-2">
            ${brand}
            ${material}
            ${weight}
          </div>
        </div>
      </div>
    `;
  }

  function renderMarkers(findings: Finding[]) {
    if (!L || !markersLayer) return;
    markersLayer.clearLayers();

    findings.forEach((f) => {
      const m = L.marker([f.lat, f.lng], {
        title: f.category,
        icon: delitterIcon
      });

      m.bindPopup(popupHtml(f), {
        className: 'dl-popup',
        maxWidth: 320,
        offset: [0, -6]
      });

      m.addTo(markersLayer);
    });

    if (findings.length > 0) {
      const bounds = L.latLngBounds(findings.map((f) => [f.lat, f.lng]));
      map.fitBounds(bounds.pad(0.2), { animate: false });
    }
  }

  // load findings from backend
  async function loadFindings() {
    const BASE = (PUBLIC_BACKEND_URL ?? '').replace(/\/+$/, '');
    const token = (auth.getToken?.() ?? get(auth)) as string | null;

    const url = `${BASE}/protected/litter`; // NOTE: /v1 is already in BASE
    const res = await fetch(url, {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${localStorage.getItem('jwt')}`
      }
    });

    if (!res.ok) throw new Error(`GET ${url} -> ${res.status}`);

    const apiData: Array<{
      id?: string;
      lat: number;
      lng: number;
      weight?: number | null;
      category: string;
      material: string;
      brand?: string | null;
    }> = await res.json();

    const items: Finding[] = apiData.map((d) => ({
      id: d.id ?? `${d.lat},${d.lng}`,
      lat: d.lat,
      lng: d.lng,
      weight: d.weight ?? null,
      category: d.category,
      material: d.material,
      brand: d.brand ?? null
    }));
    console.log('Loaded findings from backend:', items);
    renderMarkers(items);
  }

  // geolocation watch: pulsing dot + accuracy circle
  function startLocateWatch(centerOnce = true) {
    if (!browser || !map || !navigator.geolocation) return;
    if (geoWatchId) navigator.geolocation.clearWatch(geoWatchId);

    const userIcon = L.divIcon({
      className: 'dl-user',
      html: '<span class="pulse"></span><span class="dot"></span>',
      iconSize: [16, 16],
      iconAnchor: [8, 8]
    });

    geoWatchId = navigator.geolocation.watchPosition(
      (pos) => {
        const { latitude, longitude, accuracy } = pos.coords;
        const ll: [number, number] = [latitude, longitude];

        if (!userMarker) userMarker = L.marker(ll, { icon: userIcon }).addTo(map);
        else userMarker.setLatLng(ll);

        if (!userAccuracy) {
          userAccuracy = L.circle(ll, {
            radius: accuracy,
            color: '#3b82f6',
            weight: 1,
            fillColor: '#3b82f6',
            fillOpacity: 0.1
          }).addTo(map);
        } else {
          userAccuracy.setLatLng(ll);
          userAccuracy.setRadius(accuracy);
        }

        if (centerOnce) {
          map.setView(ll, 15);
          centerOnce = false;
        }
      },
      () => {},
      { enableHighAccuracy: true, maximumAge: 10000, timeout: 10000 }
    );
  }

  function stopLocateWatch() {
    if (geoWatchId) {
      navigator.geolocation.clearWatch(geoWatchId);
      geoWatchId = null;
    }
    if (userMarker) { map.removeLayer(userMarker); userMarker = null; }
    if (userAccuracy) { map.removeLayer(userAccuracy); userAccuracy = null; }
  }

  // one-click helper
  function locateMe() { startLocateWatch(true); }

  onMount(async () => {
    if (!browser) return;
    const mod = await import('leaflet');
    L = mod.default;

    // custom SVG pin (ensure it exists in /static/icons/)
    delitterIcon = L.icon({
      iconUrl: '/icons/litter-pin.svg',
      iconSize: [28, 40],
      iconAnchor: [14, 40],
      popupAnchor: [0, -36]
    });

    map = L.map(mapDiv, {
      center: [47.5596, 7.5886],
      zoom: 13,
      minZoom: 3,
      maxZoom: 19,
      zoomControl: true
    });

    const cartoVoyager = L.tileLayer(
      'https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png',
      {
        maxZoom: 20,
        attribution: '&copy; OpenStreetMap contributors | &copy; <a href="https://carto.com/attributions">CARTO</a>'
      }
    );
    cartoVoyager.addTo(map);

    markersLayer = L.layerGroup().addTo(map);

    try {
      await loadFindings();         // ← now loads from backend
    } catch (e) {
      console.warn('Backend fetch failed, showing demo markers.', e);
      renderMarkers(demoFindings);  // fallback
    }

    locateMe(); // optional geolocate
  });

  onDestroy(() => {
    if (map) map.remove();
    stopLocateWatch();
  });
</script>

<div class="page h-screen flex flex-col bg-base-100">
  <div class="toolbar flex items-center gap-2 p-2 border-b border-base-200">
    <button class="btn btn-sm btn-accent" on:click={locateMe}>My location</button>
    <button class="btn btn-sm" on:click={stopLocateWatch}>Stop</button>
  </div>

  <div class="map grow min-h-[60vh]" bind:this={mapDiv} aria-label="Delitter map"></div>
</div>

<style>
  /* Leaflet container sizing */
  :global(.leaflet-container) {
    width: 100%;
    height: 100%;
    font-family: system-ui, sans-serif;
  }

  /* Popup wrapper → let the daisyUI card define size */
  :global(.dl-popup .leaflet-popup-content){margin:0;padding:0;}
  :global(.dl-popup .leaflet-popup-content-wrapper){
    background: var(--color-base-100);
    border-radius: .75rem;
    border: 1px solid color-mix(in oklch, var(--color-base-200) 60%, transparent);
    box-shadow: 0 8px 24px color-mix(in oklch, var(--color-base-content) 8%, transparent);
    padding: 0;
  }
  :global(.dl-popup .leaflet-popup-tip){
    background: var(--color-base-100);
    box-shadow: 0 8px 24px color-mix(in oklch, var(--color-base-content) 6%, transparent);
  }
  
  /* “My location” pulsing dot */
  :global(.dl-user){position:relative;}
  :global(.dl-user .dot){
    position:absolute;left:50%;top:50%;transform:translate(-50%,-50%);
    width:10px;height:10px;border-radius:9999px;
    background: var(--color-accent);
    border:2px solid var(--color-base-100);
    box-shadow:0 1px 4px color-mix(in oklch, var(--color-base-content) 25%, transparent);
  }
  :global(.dl-user .pulse){
    position:absolute;left:50%;top:50%;transform:translate(-50%,-50%);
    width:18px;height:18px;border-radius:9999px;
    background: color-mix(in oklch, var(--color-accent) 35%, transparent);
    animation: dl-pulse 1.6s ease-out infinite;
  }
  @keyframes dl-pulse{
    0%{transform:translate(-50%,-50%) scale(.4);opacity:.7}
    70%{transform:translate(-50%,-50%) scale(1.4);opacity:.05}
    100%{opacity:0}
  }
</style>
