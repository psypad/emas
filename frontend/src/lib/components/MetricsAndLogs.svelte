<script lang="ts">
  import { onMount } from 'svelte';

  // Dummy data for metrics
  let totalApiCalls: number = 0;
  let successfulCalls: number = 0;
  let failedCalls: number = 0;
  let avgLatencyMs: number = 0;
  let isLoadingMetrics: boolean = true;
  let metricsError: string | null = null;

  // Dummy data for logs
  let recentLogs: Array<{ id: string; timestamp: string; message: string; level: string }> = [];
  let isLoadingLogs: boolean = true;
  let logsError: string | null = null;

  onMount(async () => {
    // Simulate fetching metrics
    isLoadingMetrics = true;
    metricsError = null;
    try {
      // TODO: Replace with actual API call for metrics
      const metricsResponse = await fetch('http://localhost:3000/api/metrics/summary');
      if (!metricsResponse.ok) {
        throw new Error(`HTTP error! status: ${metricsResponse.status}`);
      }
      const metricsData = await metricsResponse.json();
      totalApiCalls = metricsData.totalApiCalls || 123456; // Dummy value
      successfulCalls = metricsData.successfulCalls || 123000; // Dummy value
      failedCalls = metricsData.failedCalls || 456; // Dummy value
      avgLatencyMs = metricsData.avgLatencyMs || 75; // Dummy value
    } catch (e: any) {
      metricsError = e.message;
      console.error('Error fetching metrics:', e);
    } finally {
      isLoadingMetrics = false;
    }

    // Simulate fetching logs
    isLoadingLogs = true;
    logsError = null;
    try {
      // TODO: Replace with actual API call for recent logs
      const logsResponse = await fetch('http://localhost:3000/api/logs/recent');
      if (!logsResponse.ok) {
        throw new Error(`HTTP error! status: ${logsResponse.status}`);
      }
      const logsData = await logsResponse.json();
      recentLogs = logsData.logs || [ // Dummy logs
        { id: 'log1', timestamp: new Date().toLocaleString(), message: 'API call to /users successful.', level: 'INFO' },
        { id: 'log2', timestamp: new Date(Date.now() - 60000).toLocaleString(), message: 'Authentication failed for user 123.', level: 'WARN' },
        { id: 'log3', timestamp: new Date(Date.now() - 120000).toLocaleString(), message: 'Rate limit exceeded for IP 192.168.1.1', level: 'ERROR' },
      ];
    } catch (e: any) {
      logsError = e.message;
      console.error('Error fetching logs:', e);
    } finally {
      isLoadingLogs = false;
    }
  });
</script>

<div class="w-full p-4 bg-white rounded-lg">
  <h2 class="text-2xl font-semibold text-gray-800 mb-4">Metrics and Logs</h2>

  <!-- Metrics Section -->
  <div class="mb-6">
    <h3 class="text-xl font-medium text-gray-700 mb-3">API Usage Metrics</h3>
    {#if isLoadingMetrics}
      <p class="text-gray-600">Loading metrics...</p>
    {:else if metricsError}
      <p class="text-red-600">Error loading metrics: {metricsError}</p>
    {:else}
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
        <div class="bg-blue-50 p-4 rounded-lg shadow-sm">
          <p class="text-sm text-gray-600">Total API Calls</p>
          <p class="text-2xl font-bold text-blue-800">{totalApiCalls.toLocaleString()}</p>
        </div>
        <div class="bg-green-50 p-4 rounded-lg shadow-sm">
          <p class="text-sm text-gray-600">Successful Calls</p>
          <p class="text-2xl font-bold text-green-800">{successfulCalls.toLocaleString()}</p>
        </div>
        <div class="bg-red-50 p-4 rounded-lg shadow-sm">
          <p class="text-sm text-gray-600">Failed Calls</p>
          <p class="text-2xl font-bold text-red-800">{failedCalls.toLocaleString()}</p>
        </div>
        <div class="bg-yellow-50 p-4 rounded-lg shadow-sm">
          <p class="text-sm text-gray-600">Avg. Latency</p>
          <p class="text-2xl font-bold text-yellow-800">{avgLatencyMs} ms</p>
        </div>
      </div>
      <p class="text-sm text-gray-500 mt-3">Charts would typically go here for historical data.</p>
    {/if}
  </div>

  <!-- Logs Section -->
  <div>
    <h3 class="text-xl font-medium text-gray-700 mb-3">Recent API Logs</h3>
    {#if isLoadingLogs}
      <p class="text-gray-600">Loading logs...</p>
    {:else if logsError}
      <p class="text-red-600">Error loading logs: {logsError}</p>
    {:else}
      <div class="bg-gray-50 p-4 rounded-lg shadow-inner max-h-64 overflow-y-auto">
        {#if recentLogs.length === 0}
          <p class="text-gray-600 text-center">No recent logs found.</p>
        {:else}
          {#each recentLogs as log (log.id)}
            <div class="text-sm mb-2 pb-2 border-b border-gray-200 last:border-b-0">
              <span class="font-mono text-gray-500 mr-2">[{log.timestamp}]</span>
              <span class="font-semibold" class:text-green-600={log.level === 'INFO'} class:text-yellow-600={log.level === 'WARN'} class:text-red-600={log.level === 'ERROR'}>
                {log.level}:
              </span>
              <span class="text-gray-800">{log.message}</span>
            </div>
          {/each}
        {/if}
      </div>
      <p class="text-sm text-gray-500 mt-3">This is a simplified view. A full log viewer would have filtering and pagination.</p>
    {/if}
  </div>
</div>
