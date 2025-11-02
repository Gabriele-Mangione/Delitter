<script lang="ts">
    import {onMount} from 'svelte';
    import {PUBLIC_BACKEND_URL} from "$env/static/public";
    import type {Finding, FindingEntry} from "$lib/types/finding";

    let items: HistoryEntry[] = [];
    let error = '';
    let loading = true;

    type HistoryEntry = {
        finding: Finding
        image_url: string;
        date: Date;
    };

    function formatDate(date: Date): string {
        const now = new Date();
        const pad = (n: number) => n.toString().padStart(2, '0');

        const isSameDay = (d1: Date, d2: Date) =>
            d1.getFullYear() === d2.getFullYear() &&
            d1.getMonth() === d2.getMonth() &&
            d1.getDate() === d2.getDate();

        const yesterday = new Date(now);
        yesterday.setDate(now.getDate() - 1);

        const time = `${pad(date.getHours())}:${pad(date.getMinutes())}`;

        if (isSameDay(date, now)) {
            return time;
        } else if (isSameDay(date, yesterday)) {
            return `Yesterday, ${time}`;
        } else {
            return `${pad(date.getDate())}.${pad(date.getMonth() + 1)}.${String(
                date.getFullYear()
            ).slice(2)} ${time}`;
        }
    }


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
            console.log({findings})

            items = findings.map((item: Finding) => {
                const itemClone = {...item}
                const bytes = new Uint8Array(itemClone.file)
                const blob = new Blob([bytes], {type: 'image/jpeg'})
                const url = URL.createObjectURL(blob)

                // Fix date
                const fixed = itemClone.date.replace(" +00:00:00", "Z"); // convert to UTC

                return {
                    finding: itemClone,
                    image_url: url,
                    date: new Date(fixed)
                }
            })
                .sort((a: HistoryEntry, b: HistoryEntry) => b.date.getTime() - a.date.getTime());
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
    {#if items.length == 0}
        <div class="flex flex-col gap-2 items-center mt-8">
            <figure class="flex flex-col max-w-[200px] w-auto">
                <img class="" src="/src/lib/assets/history_empty.png" alt="Garbage">
            </figure>
            <div class="flex flex-col text-center mt-8">
                <span class="text-xl">You have not recorded any litter yet.</span>
                <span class="text-sm mt-1">Go to the recording tab to take some pictures!</span>
            </div>
        </div>
    {/if}
    <ul class="list bg-base-100 rounded-box shadow-md">
        {#each items as item (item.finding.id)}
            <li class="list-row">
                <div>
                    <img class="w-40 rounded-box" src="{item.image_url}"/>
                </div>
                <div class="flex flex-col justify-between">
                    {#if item.finding.entries.length === 0}
                        <div>
                            <div class="flex flex-row gap-1 font-bold">
                                <span>No litter detected in image.</span>
                            </div>
                        </div>
                    {:else}
                        <div>
<!--                            <div class="flex flex-row gap-1 font-bold">-->
<!--                                <span>{item.finding.entries.length}</span><span>items</span>-->
<!--                            </div>-->
                            <ul class="flex flex-col gap-3">
                                {#each item.finding.entries as entry, i (i)}
                                    <li class="flex flex-col p-0 gap-0.5">
                                        <p class="block font-bold">{i+1} {entry.category ?? 'unknown'}</p>
                                        <div class="flex flex-row flex-wrap leading-none gap-1">
                                            {#if entry.brand}
                                                <div class="badge badge-md badge-ghost">{entry.brand}</div>
                                            {/if}
                                            {#if entry.material}
                                                <div class="badge badge-md badge-ghost">{entry.material}</div>
                                            {/if}
                                            {#if entry.weight && entry.weight !== 0}
                                                <div class="badge badge-md badge-ghost">{entry.weight}g</div>
                                            {/if}
                                        </div>
                                    </li>
                                {/each}
                            </ul>
                        </div>
                        <div class="flex flex-row gap-1 opacity-60 mt-2">
                            <span>{formatDate(item.date)}</span>
                        </div>
                    {/if}
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
