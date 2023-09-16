<script>
    export let msg = "";

    export function clear() {
        msg = "";
    }
</script>

<div class="flex text-sm items-stretch">
    <div
        class="{msg
            ? 'bg-orange-600'
            : 'bg-green-800 rounded-r'} flex items-center font-medium pl-0.5 pr-1.5 py-0.5 rounded-l select-none w-min">
        {#if msg}
            <!-- This svg is from Feather Icons (https://feathericons.com). Copyright (c) 2013-2017 Cole Bemis -->
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.75"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="feather feather-alert-circle py-0.5">
                <circle cx="12" cy="12" r="10" />
                <line x1="12" y1="8" x2="12" y2="12" />
                <line x1="12" y1="16" x2="12.01" y2="16" />
            </svg>
            <span>Error</span>
        {:else}
            <!-- This svg is from Feather Icons (https://feathericons.com). Copyright (c) 2013-2017 Cole Bemis -->
            <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.75"
                stroke-linecap="round"
                stroke-linejoin="round"
                class="feather feather-check py-0.5">
                <polyline points="20 6 9 17 4 12" />
            </svg>
            <span>Valid</span>
        {/if}
    </div>

    <div class="{msg ? '' : 'hidden'} bg-gray-800 px-1.5 rounded-r flex">
        <span class="my-auto">{msg}</span>
    </div>
</div>
