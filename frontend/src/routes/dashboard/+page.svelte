<script lang="ts">
    import Footer from "../../components/footer.svelte";
    import DashboardHeader from "../../components/Dashboard-Header.svelte";

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

<div class = "page-container">
    <DashboardHeader />

    <main>
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
    </main>

    <Footer />
</div>

<style>
    
    .page-container {
        display: flex;
        flex-direction: column;
        min-height: 100vh; /* Set the container's minimum height to 100% of the viewport height */
    }

    main {
        flex: 1; /* This is the key: it makes the main area grow to fill any available space */
    }

    .space{
        padding: 10px;
        justify-self: start;
    }
    .padlef{
        padding: 10px;
        justify-self: start;
    }
</style>

