<script lang="ts">
  import { onMount } from 'svelte';
  import { PUBLIC_BACKEND_URL } from '$env/static/public';
  import { auth } from '$lib/stores/auth';
  import { get } from 'svelte/store';

  // types 
  type Finding = {
    id: string;
    lat: number;
    lng: number;
    weight: number | null;          // grams
    category: string;
    material: string;
    brand?: string | null;
    createdAt: string;
  };

  // backend record + entries (new format)
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

  // ---------- Demo fallback ----------
  const demo: Finding[] = [
    { id:'1', lat:47.5596, lng:7.5886, weight:24,  category:'Beverage Can',  material:'Aluminium', brand:'Smirnoff', createdAt:'2025-10-28' },
    { id:'2', lat:47.56,   lng:7.59,   weight:18,  category:'Snack Wrapper', material:'Plastic',   brand:'Doritos', createdAt:'2025-10-29' },
    { id:'3', lat:47.558,  lng:7.592,  weight:220, category:'Plastic Bottle', material:'Plastic',   brand:'Fanta',   createdAt:'2025-11-01' },
    { id:'4', lat:47.557,  lng:7.585,  weight:9,   category:'Paper Cup',     material:'Paper',     brand:'Café',    createdAt:'2025-11-01' },
    { id:'5', lat:47.561,  lng:7.586,  weight:15,  category:'Beverage Can',  material:'Aluminium', brand:'Red Bull',createdAt:'2025-11-02' },
    { id:'6', lat:47.559,  lng:7.587,  weight:2,   category:'Cigarette Butt',material:'Other',     brand:'Coca Cola', createdAt:'2025-11-02' },
    { id:'7', lat:47.559,  lng:7.589,  weight:240, category:'Glass Bottle',   material:'Glass',     brand:'Local',   createdAt:'2025-11-03' },
    { id:'8', lat:47.559,  lng:7.589,  weight:6,   category:'Bag',            material:'Plastic',   brand:'Coca Cola', createdAt:'2025-11-03' },
  ];

  // ---------- State ----------
  let findings: Finding[] = [];
  let loading = true;
  let loadError: string | null = null;

  type Range = '7d' | '30d' | 'all';
  let range: Range = '30d';

  // ---------- Backend fetch (flatten entries) ----------
  async function loadFindings() {
    const BASE = (PUBLIC_BACKEND_URL ?? '').replace(/\/+$/, '');
    const token = (auth.getToken?.() ?? get(auth)) as string | null;

    try {
      const res = await fetch(`${BASE}/protected/litter`, {
        headers: { Accept: 'application/json', ...(token ? { Authorization: `Bearer ${token}` } : {}) },
        cache: 'no-store'
      });
      if (!res.ok) throw new Error(`GET /protected/litter -> ${res.status}`);

      const apiData: BackendRecord[] = await res.json();

      const flat: Finding[] = [];
      for (const rec of apiData ?? []) {
        const lat = Number((rec as any).lat);
        const lng = Number((rec as any).lng);
        if (!isFinite(lat) || !isFinite(lng)) continue;

        const baseId = String(rec.id ?? `${lat},${lng}`);
        const dateStr = normalizeDateStr(rec.date ?? new Date().toISOString());
        const entries = Array.isArray(rec.entries) ? rec.entries : [];

        for (let idx = 0; idx < entries.length; idx++) {
          const e = entries[idx] ?? {};
          flat.push({
            id: `${baseId}-${idx}`,
            lat, lng,
            weight: e?.weight ?? null,
            category: (e?.category ?? 'Unknown') || 'Unknown',
            material: (e?.material ?? 'Unknown') || 'Unknown',
            brand: e?.brand ?? null,
            createdAt: dateStr
          });
        }
      }

      findings = flat.length ? flat : demo;
      loadError = flat.length ? null : 'No entries from API (showing demo)';
    } catch (e: any) {
      console.warn('Backend fetch failed, using demo data.', e);
      loadError = e?.message ?? String(e);
      findings = demo;
    } finally {
      loading = false;
    }
  }

  onMount(loadFindings);

  // helpers
  function normalizeDateStr(s: string): string {
    if (!s) return new Date().toISOString().slice(0,10);
    if (/^\d{4}-\d{2}-\d{2}$/.test(s)) return s;
    const d = new Date(s);
    if (isNaN(+d)) return new Date().toISOString().slice(0,10);
    return d.toISOString().slice(0,10);
  }
  function parseLocalDate(dateStr: string): Date {
    return new Date(dateStr + 'T12:00:00Z');
  }
  function daysAgo(n: number): string {
    const d = new Date();
    d.setUTCHours(12,0,0,0);
    d.setUTCDate(d.getUTCDate() - n);
    return d.toISOString().slice(0,10);
  }

  function inRange(d: string, r: Range): boolean {
    if (r === 'all') return true;
    const cutoffDays = r === '7d' ? 7 : 30;
    const cutoff = parseLocalDate(daysAgo(cutoffDays - 1)); // inkl. heute
    return parseLocalDate(d) >= cutoff;
  }

  $: filtered = findings.filter((f) => inRange(f.createdAt, range));

  // totals
  $: totalItems = filtered.length;
  $: totalWeightG = filtered.reduce((s, d) => s + (d.weight ?? 0), 0);
  $: totalWeightKg = Math.round(totalWeightG) / 1000;
  $: savedCO2 = +(totalWeightKg * 1.6).toFixed(2); // Mock-Faktor

  // top-by helper
  function topBy<T extends keyof Finding>(arr: Finding[], key: T) {
    const map = new Map<string, number>();
    for (const d of arr) {
      const k = String(d[key] ?? '—');
      map.set(k, (map.get(k) ?? 0) + 1);
    }
    let best = '—'; let bestN = 0;
    for (const [k, v] of map) if (v > bestN) { best = k; bestN = v; }
    return { name: best, count: bestN, map };
  }

  $: topCategory = topBy(filtered, 'category');
  $: topMaterial = topBy(filtered, 'material');
  $: topBrands = Array.from(topBy(filtered, 'brand').map.entries())
    .filter(([b]) => b && b !== 'null')
    .sort((a,b) => b[1]-a[1])
    .slice(0,5);

  // weekday activity bars
  const weekdayLabels = ['Sun','Mon','Tue','Wed','Thu','Fri','Sat'];
  $: (function(){
    const raw = Array(7).fill(0) as number[];
    for (const d of filtered) {
      const day = parseLocalDate(d.createdAt).getUTCDay();
      raw[day]++;
    }
    weekdayCountsRaw = raw;
  })();
  let weekdayCountsRaw: number[] = Array(7).fill(0);
  $: weekdayCounts = weekdayCountsRaw.map(n => n === 0 ? 1 : n); // Baseline 1 für Sichtbarkeit
  $: weekdayMax = Math.max(1, ...weekdayCountsRaw);
  const todayIdx = new Date().getUTCDay();

  // last N days trendline
  const lastDays = 8;
  $: dayCounts = Array.from({ length: lastDays }, (_, idx) => {
    const dayKey = daysAgo(lastDays - 1 - idx);
    return filtered.filter(d => d.createdAt === dayKey).length;
  });

  // sparkline path
  function sparklinePath(values: number[], w=320, h=96, pad=10) {
    const max = Math.max(1, ...values);
    const step = (w - pad*2) / Math.max(1, values.length - 1);
    const pts = values.map((v,i) => {
      const x = pad + i*step;
      const y = h - pad - (v/max)*(h - pad*2);
      return [x,y] as const;
    });
    return pts.map((p,i) => (i===0?'M':'L') + p[0].toFixed(1) + ' ' + p[1].toFixed(1)).join(' ');
  }

  // materials donut
  $: materialEntries = Array.from(topMaterial.map.entries()).sort((a,b)=>b[1]-a[1]);
  $: materialTotal = materialEntries.reduce((s, [,n]) => s+n, 0);
  type Seg = { label: string; percent: number; colorVar: string; };

  // robust color handling
  const paletteVars = ['--p','--a','--su','--in','--se','--wa','--er'];

  $: donutSegs = materialEntries.map(([label, n], i) => ({
    label,
    percent: materialTotal ? (n/materialTotal)*100 : 0,
    colorVar: paletteVars[i % paletteVars.length]
  }));

  function conicStyle(segs: Seg[]) {
    if (!segs.length) {
      return 'conic-gradient(hsl(var(--b3)) 0 100%)';
    }
    let start = 0, parts: string[] = [];
    for (const s of segs) {
      const end = start + s.percent;
      parts.push(`hsl(var(${s.colorVar})) ${start}% ${end}%`);
      start = end;
    }
    return `conic-gradient(${parts.join(',')})`;
  }

  // github style contribution heatmap
  const WEEKS = 12;
  const MS_DAY = 24 * 3600 * 1000;
  function iso(d: Date) { return d.toISOString().slice(0,10); }

  type Cell = { date: string; count: number; level: number; };
  let heatmap: Cell[][] = [];
  let monthLabels: string[] = [];

  $: (function buildHeatmap(){
    const countsByDate = new Map<string, number>();
    for (const d of filtered) {
      countsByDate.set(d.createdAt, (countsByDate.get(d.createdAt) ?? 0) + 1);
    }

    const today = new Date();
    today.setUTCHours(12,0,0,0);
    const start = new Date(today);
    const offsetToSunday = start.getUTCDay(); // Sun=0
    start.setTime(start.getTime() - (offsetToSunday + (WEEKS-1)*7) * MS_DAY);

    const h: Cell[][] = [];
    let gMax = 0;
    for (let w = 0; w < WEEKS; w++) {
      const col: Cell[] = [];
      for (let d = 0; d < 7; d++) {
        const cur = new Date(start.getTime() + (w*7 + d) * MS_DAY);
        const key = iso(cur);
        const count = countsByDate.get(key) ?? 0;
        gMax = Math.max(gMax, count);
        col.push({ date: key, count, level: 0 });
      }
      h.push(col);
    }

    const q1 = Math.max(1, Math.ceil(gMax * 0.25));
    const q2 = Math.max(2, Math.ceil(gMax * 0.50));
    const q3 = Math.max(3, Math.ceil(gMax * 0.75));
    for (const week of h) {
      for (const c of week) {
        if (c.count === 0) c.level = 0;
        else if (c.count <= q1) c.level = 1;
        else if (c.count <= q2) c.level = 2;
        else if (c.count <= q3) c.level = 3;
        else c.level = 4;
      }
    }

    heatmap = h;

    const labels: string[] = [];
    for (let w = 0; w < WEEKS; w++) {
      const firstOfCol = heatmap[w][0].date;
      const m = new Date(firstOfCol + 'T12:00:00Z');
      labels.push(m.getUTCDate() <= 7 ? m.toLocaleString(undefined, { month: 'short' }) : '');
    }
    monthLabels = labels;
  })();
</script>

<svelte:head><title>Insights • Delitter</title></svelte:head>

<div class="space-y-6">
  <div class="flex items-center justify-between gap-3">
    <h1 class="text-2xl md:text-3xl font-semibold">Insights</h1>
    <div class="join">
      <button class="btn btn-sm join-item btn-ghost" class:btn-active={range==='7d'} on:click={() => range='7d'}>7d</button>
      <button class="btn btn-sm join-item btn-ghost" class:btn-active={range==='30d'} on:click={() => range='30d'}>30d</button>
      <button class="btn btn-sm join-item btn-ghost" class:btn-active={range==='all'} on:click={() => range='all'}>All</button>
    </div>
  </div>

  {#if loading}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 animate-pulse">
      {#each Array(4) as _}
        <div class="card bg-base-100 shadow-sm"><div class="card-body">
          <div class="h-4 w-24 bg-base-300/60 rounded"></div>
          <div class="h-8 w-32 bg-base-300/80 rounded mt-2"></div>
          <div class="h-3 w-20 bg-base-300/60 rounded mt-2"></div>
        </div></div>
      {/each}
    </div>
  {:else}
    {#if loadError}
      <div class="alert alert-warning">
        <span>{loadError}</span>
      </div>
    {/if}

    <!-- stats -->
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
      <div class="card bg-base-100 shadow-sm"><div class="card-body">
        <div class="flex items-center justify-between">
          <span class="text-base-content/70">Items collected</span>
          <div class="badge">{topCategory.name}</div>
        </div>
        <div class="text-3xl font-bold">{totalItems}</div>
        <div class="text-xs text-base-content/60">Top category • {topCategory.count}</div>
      </div></div>

      <div class="card bg-base-100 shadow-sm"><div class="card-body">
        <div class="flex items-center justify-between">
          <span class="text-base-content/70">Total weight</span>
          <div class="badge badge-success badge-outline">{totalWeightG} g</div>
        </div>
        <div class="text-3xl font-bold">{totalWeightKg.toFixed(2)} kg</div>
        <div class="text-xs text-base-content/60">Est. CO₂ saved ~ {savedCO2} kg</div>
      </div></div>

      <div class="card bg-base-100 shadow-sm"><div class="card-body">
        <div class="text-base-content/70">Top material</div>
        <div class="text-3xl font-bold">{topMaterial.name}</div>
        <div class="text-xs text-base-content/60">{topMaterial.count} findings</div>
      </div></div>

      <div class="card bg-base-100 shadow-sm"><div class="card-body">
        <div class="text-base-content/70">Active volunteers</div>
        <div class="text-3xl font-bold">12</div>
        <div class="text-xs text-base-content/60">Mock value (coming soon)</div>
      </div></div>
    </div>

    <!-- charts -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
      <!-- sparkline -->
      <div class="card bg-base-100 shadow-sm lg:col-span-2">
        <div class="card-body">
          <div class="flex items-center justify-between">
            <h2 class="card-title">Daily trend</h2>
            <span class="text-xs text-base-content/60">last {dayCounts.length} days</span>
          </div>
          <div class="mt-2">
            <svg viewBox="0 0 320 96" class="w-full h-24">
              <defs>
                <linearGradient id="g1" x1="0" x2="0" y1="0" y2="1">
                  <stop offset="0%"  stop-color="hsl(var(--a))" stop-opacity="0.6"></stop>
                  <stop offset="100%" stop-color="hsl(var(--a))" stop-opacity="0.02"></stop>
                </linearGradient>
              </defs>
              <path d="{sparklinePath(dayCounts, 320, 96, 10)} L 310 96 L 10 96 Z" fill="url(#g1)"></path>
              <path d="{sparklinePath(dayCounts, 320, 96, 10)}" fill="none" stroke="hsl(var(--a))" stroke-width="2.5" stroke-linecap="round"></path>
            </svg>
          </div>
          <div class="flex gap-3 text-xs text-base-content/60">
            <div class="badge badge-outline">Peak: {Math.max(...dayCounts)}</div>
            <div class="badge badge-outline">Avg: {(dayCounts.reduce((s,n)=>s+n,0)/Math.max(1,dayCounts.length)).toFixed(1)}</div>
          </div>
        </div>
      </div>

      <!-- materials donut -->
      <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h2 class="card-title">Materials</h2>
          <div class="flex items-center gap-6">
            <!-- donut pie -->
            <div class="relative">
              <div
                class="w-28 h-28 rounded-full border border-base-300"
                style:background={conicStyle(donutSegs)}
                title="Material breakdown"
              ></div>
              <div class="absolute inset-3 rounded-full bg-base-100 border border-base-300/30"></div>
            </div>

            <div class="grow space-y-2">
              {#if donutSegs.length === 0}
                <div class="text-sm text-base-content/60">No data.</div>
              {:else}
                {#each donutSegs as s}
                  <div class="flex items-center justify-between text-sm">
                    <div class="flex items-center gap-2">
                      <span class="w-3 h-3 rounded-full" style={`background:hsl(var(${s.colorVar}))`}></span>
                      <span>{s.label}</span>
                    </div>
                    <span class="text-base-content/70">{s.percent.toFixed(0)}%</span>
                  </div>
                {/each}
              {/if}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- contribution heatmap -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div class="flex items-center justify-between">
          <h2 class="card-title">Activity heatmap</h2>
          <span class="text-xs text-base-content/60">last {WEEKS} weeks</span>
        </div>

        <div class="overflow-x-auto">
          <div class="pl-8 flex gap-1 mb-1">
            {#each monthLabels as label}
              <div class="w-3 text-[10px] text-base-content/50">{label}</div>
            {/each}
          </div>

          <div class="flex items-start gap-2">
            <!-- day labels -->
            <div class="flex flex-col justify-between text-[10px] text-base-content/60 h-[6.5rem] py-[2px]">
              <span>Mon</span>
              <span>Wed</span>
              <span>Fri</span>
            </div>

            <!-- weeks (columns) -->
            <div class="flex gap-1">
              {#each heatmap as week}
                <div class="flex flex-col gap-1">
                  {#each week as cell}
                    <div
                      class={`w-3 h-3 rounded-sm ${cell.level === 0 ? 'bg-base-300/30' : cell.level === 1 ? 'bg-accent/25' : cell.level === 2 ? 'bg-accent/45' : cell.level === 3 ? 'bg-accent/65' : 'bg-accent'}`}
                      title={`${cell.date}: ${cell.count} finding${cell.count===1?'':'s'}`}
                    ></div>
                  {/each}
                </div>
              {/each}
            </div>
          </div>
        </div>

        <div class="mt-3 flex items-center gap-2 text-xs text-base-content/60">
          <span>Less</span>
          <div class="flex gap-1">
            <span class="w-3 h-3 rounded-sm bg-base-300/30"></span>
            <span class="w-3 h-3 rounded-sm bg-accent/25"></span>
            <span class="w-3 h-3 rounded-sm bg-accent/45"></span>
            <span class="w-3 h-3 rounded-sm bg-accent/65"></span>
            <span class="w-3 h-3 rounded-sm bg-accent"></span>
          </div>
          <span>More</span>
        </div>
      </div>
    </div>

    <!-- weekday bars + brands -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
      <div class="card bg-base-100 shadow-sm lg:col-span-2">
        <div class="card-body">
          <h2 class="card-title">Activity by weekday</h2>
          <div class="mt-3 grid grid-cols-7 gap-3 items-end h-44">
            {#each weekdayCounts as n, i}
              <!-- WICHTIG: h-full, damit Prozenthöhe referenziert werden kann -->
              <div class="flex flex-col items-center gap-2 h-full">
                <div
                  class={`rounded-t-md transition-all min-h-[8px] w-6 md:w-8 ${i===todayIdx ? 'bg-accent' : 'bg-accent/70'}`}
                  style={`height:${Math.max(8, (n/Math.max(1, weekdayMax)) * 100)}%`}
                  title={`${weekdayLabels[i]}: ${weekdayCountsRaw[i]}`}
                ></div>
                <div class="flex flex-col items-center leading-tight">
                  <span class="text-xs text-base-content/60">{weekdayLabels[i]}</span>
                  <span class="badge badge-ghost badge-xs">{weekdayCountsRaw[i]}</span>
                </div>
              </div>
            {/each}
          </div>
          <div class="mt-2 text-xs text-base-content/60">*Empty days use a subtle baseline so the chart stays readable.</div>
        </div>
      </div>

      <div class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h2 class="card-title">Top brands</h2>
          <ul class="mt-2 space-y-2">
            {#if topBrands.length === 0}
              <li class="text-sm text-base-content/60">No branded litter yet.</li>
            {:else}
              {#each topBrands as [brand, cnt], idx}
                <li class="flex items-center justify-between">
                  <div class="flex items-center gap-2">
                    <div class="badge badge-outline">{idx + 1}</div>
                    <span class="font-medium">{brand}</span>
                  </div>
                  <span class="text-base-content/70">{cnt}</span>
                </li>
              {/each}
            {/if}
          </ul>

          <div class="divider my-3"></div>
          <div class="text-xs text-base-content/60">
            *Live aus dem Backend, sonst Demo.
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .card { border: 1px solid hsl(var(--b2) / .2); }
</style>
