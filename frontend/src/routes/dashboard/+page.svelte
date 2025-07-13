<script lang="ts">
    import { onMount } from "svelte";

    let status: string = "";

    type LogsResponse = {
        [endpoint: string]: string;
    };

    let logs: LogsResponse = {};


    onMount(async () => {
        const res = await fetch("http://localhost:3000/api/metrics");
        const data = await res.json();
        status = data.status;
    });

    onMount(async () => {
        const res = await fetch("http://localhost:3000/api/logs");
        logs = await res.json();
    });

</script>

<fieldset class = "space">
    <legend>Backend status:</legend>
    <p class = "padlef"> {status}</p>
</fieldset>

<fieldset class = "space">
    <legend>API Request Logs:</legend>
    <div class = "padlef">
        <ul>
        {#each Object.entries(logs) as [endpoint, status]}
            <li><code>{endpoint}</code>: <span>{status}</span></li>
        {/each}
        </ul>
    </div>
</fieldset>

<style>
    .space{
        padding: 10px;
        justify-self: start;
    }
    .padlef{
        padding: 10px;
        justify-self: start;
    }
</style>

