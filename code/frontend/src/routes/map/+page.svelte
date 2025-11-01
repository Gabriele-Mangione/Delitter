<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { browser } from '$app/environment';
  // import { base } from '$app/paths'; // use if you deploy under a base path
  import 'leaflet/dist/leaflet.css';

  type Finding = {
    id: string;
    lat: number;
    lng: number;
    weight: number | null;
    category: string;
    material: string;
    brand?: string | null;
  };

  const demoFindings: Finding[] = [
    { id: '1', lat: 47.5596, lng: 7.5886, weight: 20, category: 'Beverage Can', material: 'Aluminium', brand: 'Smirnoff' },
    { id: '2', lat: 47.5591, lng: 7.5859, weight: 20, category: 'Snack Wrapper', material: 'Plastic' },
    { id: '3', lat: 47.5584, lng: 7.5920, weight: 20, category: 'Plastic Bottle', material: 'Plastic', brand: 'Fanta' }
  ];

  let mapDiv: HTMLDivElement;
  let map: any;
  let markersLayer: any;
  let L: any;
  let delitterIcon: any;

  // --- helpers ---
  function escapeHtml(s: string) {
    return s.replace(/[&<>"']/g, (c) => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]!));
  }
  function popupHtml(f: Finding) {
    const brand = f.brand ? `<span class="badge badge-outline">${escapeHtml(f.brand)}</span>` : '';
    const material = `<span class="badge badge-success badge-outline">${escapeHtml(f.material)}</span>`;
    const weight = f.weight != null ? `<span class="text-xs text-base-content/60">${f.weight} g</span>` : '';
    return `
      <div class="card w-72 bg-base-100">
        <div class="card-body p-4">
          <h3 class="card-title text-base">${escapeHtml(f.category)}</h3>
          <div class="flex flex-wrap gap-2 items-center">
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
        className: 'dl-popup',     // lets us tweak wrapper minimally via :global CSS
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

  function locateMe() {
    if (!browser || !map || !navigator.geolocation) return;
    navigator.geolocation.getCurrentPosition(
      (pos) => map.setView([pos.coords.latitude, pos.coords.longitude], 15),
      () => {},
      { enableHighAccuracy: true, maximumAge: 30000, timeout: 5000 }
    );
  }

  onMount(async () => {
    if (!browser) return;
    const mod = await import('leaflet');
    L = mod.default;

    // custom marker / pin (ensure file exists in /static/icons/)
    delitterIcon = L.icon({
      iconUrl: '/icons/litter-pin.svg',            // or: `${base}/icons/litter-pin.svg`
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
    renderMarkers(demoFindings);
    locateMe(); // optional
  });

  onDestroy(() => { if (map) map.remove(); });
</script>

<div class="page h-screen flex flex-col bg-base-100">
  <div class="toolbar flex items-center gap-2 p-2 border-b border-base-200">
    <button class="btn btn-sm btn-accent" on:click={locateMe}>Locate me</button>
  </div>

  <div class="map grow min-h-[60vh]" bind:this={mapDiv} aria-label="Delitter map"></div>
</div>

<style>

  :global(.leaflet-container) {
    width: 100%;
    height: 100%;
    font-family: system-ui, sans-serif;
  }

  /* popup */
  :global(.dl-popup .leaflet-popup-content) { margin: 0; padding: 0; }
  :global(.dl-popup .leaflet-popup-content-wrapper) {
    background: hsl(var(--b1));
    border-radius: 0.75rem;
    border: 1px solid hsl(var(--b2) / 0.2);
    box-shadow: 0 8px 24px hsl(var(--bc) / 0.08);
    padding: 0;
  }
  :global(.dl-popup .leaflet-popup-tip) {
    background: hsl(var(--b1));
    box-shadow: 0 8px 24px hsl(var(--bc) / 0.06);
  }
</style>
