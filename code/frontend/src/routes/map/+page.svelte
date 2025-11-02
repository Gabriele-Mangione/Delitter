<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { browser } from '$app/environment';
  import 'leaflet/dist/leaflet.css';

  import MapContainer from '$lib/MapContainer.svelte';
  import { PUBLIC_BACKEND_URL } from '$env/static/public';
  import { auth } from '$lib/stores/auth';
  import { get } from 'svelte/store';

  // backend types 
  type BackendEntry = {
    category?: string | null;
    material?: string | null;
    weight?: number | null;
    brand?: string | null;
  };
  type BackendRecord = {
    id?: string;
    lat: number;
    lng: number;
    type?: string;
    file?: unknown;
    date?: string;
    entries?: BackendEntry[] | null;
  };

  // demo fallback
  const demoRecords: BackendRecord[] = [
    {
      id: 'demo-1',
      lat: 47.5596, lng: 7.5886, date: '2025-11-02',
      entries: [
        { category: 'Beverage Can', material: 'Aluminium', weight: 40, brand: 'Smirnoff' },
        { category: 'Snack Wrapper', material: 'Plastic',   weight: 20, brand: 'Doritos' }
      ]
    },
    {
      id: 'demo-2',
      lat: 47.5584, lng: 7.5920, date: '2025-11-03',
      entries: [
        { category: 'Plastic Bottle', material: 'Plastic', weight: 20, brand: 'Fanta' }
      ]
    }
  ];

  // leaflet refs
  let mapDiv: HTMLDivElement;
  let mapWrap: HTMLDivElement;       // from MapContainer (el)
  let ro: ResizeObserver | null = null;

  let map: any;
  let markersLayer: any;
  let L: any;
  let delitterIcon: any;

  // basemap layers
  let baseLight: any;
  let baseDark: any;
  let currentBase: any;
  let themeObs: MutationObserver | null = null;
  let mql: MediaQueryList | null = null;

  // user location layers
  let userMarker: any = null;
  let userAccuracy: any = null;
  let geoWatchId: number | null = null;

  // helpers 
  const esc = (s: string) =>
    s.replace(/[&<>"']/g, (c) => ({ '&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;' }[c]!));
  const fmtWeight = (w: number | null | undefined) => w == null ? '' : `${w} g`;
  const norm = (s?: string) => (s && s.trim()) || 'Unknown';

  // popup UI (navigierbar)
  function bindNavigablePopup(marker: any, rec: BackendRecord) {
  const items = (rec.entries ?? []).slice();
  const hasItems = items.length > 0;

  const el = document.createElement('div');
  el.className = 'dl-popup-content';

  // fix click propagation
  if (L?.DomEvent) {
    L.DomEvent.disableClickPropagation(el);
    L.DomEvent.disableScrollPropagation(el);
  }

  let idx = 0;

  const render = () => {
    const total = items.length;
    const cur = items[idx];

    const headerBadges = hasItems ? `
      <div class="flex flex-wrap items-center gap-2">
        ${cur?.brand ? `<span class="badge badge-outline">${esc(cur!.brand!)}</span>` : ''}
        ${cur?.material ? `<span class="badge badge-success badge-outline">${esc(cur!.material!)}</span>` : ''}
        ${cur?.weight != null ? `<span class="text-xs text-base-content/60">${fmtWeight(cur!.weight)}</span>` : ''}
      </div>
    ` : '';

    el.innerHTML = `
      <div class="card bg-base-100 text-base-content">
        <div class="card-body p-4">
          <div class="flex items-center justify-between gap-3">
            <h3 class="card-title text-base m-0">
              ${hasItems ? esc(norm(cur?.category ?? 'Unknown')) : 'No detections'}
            </h3>
            ${rec.date ? `<div class="text-xs text-base-content/60">${esc(String(rec.date).slice(0, 10))}</div>` : ''}
          </div>

          ${headerBadges}

          <div class="mt-3 flex items-center justify-between">
            <button class="btn btn-xs btn-ghost dl-nav" data-action="prev" ${total <= 1 ? 'disabled' : ''}>
              ⬅️
            </button>
            <div class="text-xs text-base-content/60">${hasItems ? `${idx + 1} / ${total}` : ''}</div>
            <button class="btn btn-xs btn-ghost dl-nav" data-action="next" ${total <= 1 ? 'disabled' : ''}>
              ➡️
            </button>
          </div>
        </div>
      </div>
    `;
  };

  // 
  el.addEventListener('click', (ev) => {
    ev.preventDefault();
    ev.stopPropagation();

    const btn = (ev.target as HTMLElement)?.closest('.dl-nav') as HTMLElement | null;
    if (!btn) return;

    const total = items.length;
    if (total <= 1) return;

    const action = btn.dataset.action;
    if (action === 'prev') idx = (idx - 1 + total) % total;
    else if (action === 'next') idx = (idx + 1) % total;

    render();
  }, { capture: true }); 

  render();

  marker.bindPopup(el, {
    className: 'dl-popup',
    maxWidth: 320,
    offset: [0, -6],
    keepInView: true
  });
}

  // render: one marker per record / location
  function renderMarkersForRecords(records: BackendRecord[]) {
    if (!L || !markersLayer) return;
    markersLayer.clearLayers();

    const pts: [number, number][] = [];

    for (const rec of records) {
      const lat = Number((rec as any).lat);
      const lng = Number((rec as any).lng);
      if (!isFinite(lat) || !isFinite(lng)) continue;

      const entries = Array.isArray(rec.entries) ? rec.entries : [];
      const title = `${entries.length || 0} detection${entries.length === 1 ? '' : 's'}`;

      const m = L.marker([lat, lng], { title, icon: delitterIcon });
      bindNavigablePopup(m, rec);
      m.addTo(markersLayer);

      pts.push([lat, lng]);
    }

    if (pts.length) {
      const bounds = L.latLngBounds(pts);
      map.fitBounds(bounds.pad(0.2), { animate: false });
    }
  }

  // fetch records from backend 
  async function loadRecordsAndRender() {
    const BASE = (PUBLIC_BACKEND_URL ?? '').replace(/\/+$/, '');
    const token = (auth.getToken?.() ?? get(auth)) as string | null;

    try {
      const res = await fetch(`${BASE}/protected/litter`, {
        headers: { Accept: 'application/json', ...(token ? { Authorization: `Bearer ${token}` } : {}) },
        cache: 'no-store'
      });
      if (!res.ok) throw new Error(`GET /protected/litter -> ${res.status}`);

      const apiData: BackendRecord[] = await res.json();
      renderMarkersForRecords(apiData ?? []);
    } catch (e) {
      console.warn('Backend fetch failed, showing demo markers.', e);
      renderMarkersForRecords(demoRecords);
    }
  }

  const isDarkTheme = () => {
    const attr = document.documentElement.getAttribute('data-theme');
    if (attr) return /dark/i.test(attr);
    return window.matchMedia?.('(prefers-color-scheme: dark)').matches ?? false;
  };

  function applyBaseLayer() {
    if (!map || !baseLight || !baseDark) return;
    const next = isDarkTheme() ? baseDark : baseLight;
    if (currentBase === next) return;
    if (currentBase && map.hasLayer(currentBase)) map.removeLayer(currentBase);
    next.addTo(map);
    currentBase = next;
    setTimeout(() => map?.invalidateSize(), 0);
  }

  function startLocateWatch(centerOnce = true) {
    if (!browser || !map || !navigator.geolocation) return;
    if (geoWatchId) navigator.geolocation.clearWatch(geoWatchId);

    const userIcon = L.divIcon({
      className: 'dl-user',
      html: '<span class="pulse"></span><span class="dot"></span>',
      iconSize: [16, 16], iconAnchor: [8, 8]
    });

    geoWatchId = navigator.geolocation.watchPosition(
      (pos) => {
        const { latitude, longitude, accuracy } = pos.coords;
        const ll: [number, number] = [latitude, longitude];

        if (!userMarker) userMarker = L.marker(ll, { icon: userIcon }).addTo(map);
        else userMarker.setLatLng(ll);

        if (!userAccuracy) {
          userAccuracy = L.circle(ll, {
            radius: accuracy, color: '#3b82f6', weight: 1, fillColor: '#3b82f6', fillOpacity: 0.1
          }).addTo(map);
        } else {
          userAccuracy.setLatLng(ll);
          userAccuracy.setRadius(accuracy);
        }

        if (centerOnce) { map.setView(ll, 15); centerOnce = false; }
      },
      () => {},
      { enableHighAccuracy: true, maximumAge: 10000, timeout: 10000 }
    );
  }
  function stopLocateWatch() {
    if (geoWatchId) { navigator.geolocation.clearWatch(geoWatchId); geoWatchId = null; }
    if (userMarker) { map.removeLayer(userMarker); userMarker = null; }
    if (userAccuracy) { map.removeLayer(userAccuracy); userAccuracy = null; }
  }
  function locateMe() { startLocateWatch(true); }

  onMount(async () => {
    if (!browser) return;
    const mod = await import('leaflet');
    L = mod.default;

    delitterIcon = L.icon({
      iconUrl: '/icons/litter-pin.svg',
      iconSize: [28, 40],
      iconAnchor: [14, 40],
      popupAnchor: [0, -36]
    });

    map = L.map(mapDiv, { center: [47.5596, 7.5886], zoom: 13, minZoom: 3, maxZoom: 19, zoomControl: true });

    baseLight = L.tileLayer(
      'https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png',
      { maxZoom: 20, attribution: '&copy; OpenStreetMap contributors | &copy; <a href="https://carto.com/attributions">CARTO</a>' }
    );
    baseDark = L.tileLayer(
      'https://{s}.basemaps.cartocdn.com/rastertiles/dark_all/{z}/{x}/{y}{r}.png',
      { maxZoom: 20, attribution: '&copy; OpenStreetMap contributors | &copy; <a href="https://carto.com/attributions">CARTO</a>' }
    );

    applyBaseLayer();
    map.attributionControl.setPrefix(false);
    map.attributionControl.setPosition('topright');

    // keep Leaflet sized to the container
    ro = new ResizeObserver(() => map?.invalidateSize());
    ro.observe(mapWrap);

    themeObs = new MutationObserver(applyBaseLayer);
    themeObs.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
    mql = window.matchMedia('(prefers-color-scheme: dark)');
    mql.addEventListener?.('change', applyBaseLayer);

    markersLayer = L.layerGroup().addTo(map);

    try { await loadRecordsAndRender(); }
    catch (e) { console.warn('Backend fetch failed, showing demo markers.', e); renderMarkersForRecords(demoRecords); }
  });

  onDestroy(() => {
    if (map) map.remove();
    stopLocateWatch();
    ro?.disconnect();
    themeObs?.disconnect();
    mql?.removeEventListener?.('change', applyBaseLayer);
  });

  // centralize these so MapContainer and the floating button stay in sync
  const NAV_H = 56;   // navbar height (px)
  const DOCK_H = 72;  // bottom Dock height (px)
</script>

<!-- No h-screen anywhere; the container owns the height -->
<MapContainer bind:el={mapWrap} top={NAV_H} bottom={DOCK_H} className="bg-base-100">
  <div class="absolute inset-0" bind:this={mapDiv} aria-label="Delitter map"></div>

  <!-- Floating “Locate me” button -->
  <div
    class="absolute right-3 z-[1001] pointer-events-auto"
    style={`bottom: calc(${DOCK_H}px + env(safe-area-inset-bottom, 0px) + 12px)`}
  >
    <button
      class="btn btn-circle btn-accent shadow-md"
      on:click={locateMe}
      aria-label="Locate me"
      title="Locate me"
    >
      <svg width="22" height="22" viewBox="0 0 24 24" fill="none" aria-hidden="true">
        <path d="M12 3v3m0 12v3M3 12h3m12 0h3M12 7a5 5 0 100 10 5 5 0 000-10z"
              stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
      </svg>
    </button>
  </div>
</MapContainer>

<style>
  :global(.leaflet-container){ width:100%; height:100%; font-family:system-ui,sans-serif; }
  :global(.leaflet-control-container){ z-index: 900; }

  /* popup */
  :global(.dl-popup .leaflet-popup-content){ margin:0; padding:0; }
  :global(.dl-popup .leaflet-popup-content-wrapper){
    background: var(--color-base-100);
    color: var(--color-base-content);
    border-radius: .75rem;
    border: 1px solid color-mix(in oklch, var(--color-base-200) 60%, transparent);
    box-shadow: 0 8px 24px color-mix(in oklch, var(--color-base-content) 8%, transparent);
    padding: 0;
  }
  :global(.dl-popup .leaflet-popup-tip){
    background: var(--color-base-100);
    box-shadow: 0 8px 24px color-mix(in oklch, var(--color-base-content) 6%, transparent);
  }

  /* user location marker */
  :global(.dl-user){ position:relative; }
  :global(.dl-user .dot){
    position:absolute;left:50%;top:50%;transform:translate(-50%,-50%);
    width:10px;height:10px;border-radius:9999px;background:var(--color-accent);
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

  :global(.dl-popup-content .card-title){ line-height: 1.2; }
</style>
