<script lang="ts">
    import {onMount} from 'svelte';
    import {PUBLIC_BACKEND_URL} from "$env/static/public";
    import type {Finding} from "$lib/types/finding";

    let items: HistoryEntry[] = [];
    let error = '';
    let loading = true;

    type HistoryEntry = {
        finding: Finding
        image_url: string;
        date: Date;
    };

    onMount(async () => {
        try {
            const base = PUBLIC_BACKEND_URL;
            const api_url = `${base}/protected/litter`
            const res = await fetch(api_url, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${localStorage.getItem('jwt')}`
                }
            });
            if (!res.ok) {
                throw new Error(await res.text())
            }
            ;
            const findings = await res.json(); // array of objects

            items = findings.map((item : Finding) => {
                // TODO: Remove
                item.brand = "Coca Cola"
                item.category = "Can"
                item.material = "Aluminum"
                item.weight = 10  // g

                const bytes = new Uint8Array(item.file)
                const blob = new Blob([bytes], { type: 'image/jpeg' })
                const url = URL.createObjectURL(blob)

                // Fix date
                const fixed = item.date.replace(" +00:00:00", "Z"); // convert to UTC

                return {
                    finding: item,
                    image_url: url,
                    date: new Date(fixed)
                }
            })
        } catch (e) {
            error = e.message;
        } finally {
            loading = false;
        }
    });
</script>

{#if loading}
    <p>Loading…</p>
{:else if error}
    <p class="error">{error}</p>
{:else}
    <ul>

    </ul>

    <ul class="list bg-base-100 rounded-box shadow-md">
        <li class="p-4 pb-2 text-xs opacity-60 tracking-wide">Most played songs this week</li>
        {#each items as item: HistoryEntry (item.finding.id)}
            <li class="list-row">
                <div>
                    <img class="w-40 rounded-box" src="{item.image_url}"/>
                </div>
                <div class="flex flex-col gap-2">
                    <div class="font-bold text-lg" >{item.finding.category}</div>
                    <div class="flex flex-col gap-1">
                        <div class="text-xs uppercase font-semibold opacity-60">{item.finding.brand}</div>
                        <div class="text-xs font-semibold opacity-60">
                            <span class="uppercase">{item.finding.weight}</span><span class="">g</span>
                        </div>
                        <div class="text-xs uppercase font-semibold opacity-60">{item.finding.material}</div>
                        <div class="text-xs uppercase font-semibold opacity-60">{item.date}</div>
                    </div>
                </div>
                <button aria-label="delete" class="btn btn-square btn-ghost">
                    <svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                         stroke="currentColor">
                        <path stroke-width="2" stroke-linecap="round" stroke-linejoin="round"
                              d="m14.74 9-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 0 1-2.244 2.077H8.084a2.25 2.25 0 0 1-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 0 0-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 0 1 3.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 0 0-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 0 0-7.5 0"/>
                    </svg>
                </button>
            </li>
        {/each}
    </ul>
{/if}
