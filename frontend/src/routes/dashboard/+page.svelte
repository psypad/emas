<script lang="ts">
  // Import the components needed for the dashboard
  import CreateApiKey from '$lib/components/CreateApiKey.svelte';
  import MetricsAndLogs from '$lib/components/MetricsAndLogs.svelte';
  import ApiKeysTable from '$lib/components/ApiKeysTable.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
</script>


<!-- SideBar needs a lotta work

<div class="dashboard-layout">
  <Sidebar />
  <main class="dashboard-content">
    <h1>Welcome to your Dashboard!</h1>
    <p>This is the main content area of your dashboard.</p>
    </main>
</div>

-->

<div class="min-h-screen bg-gray-100 p-4 sm:p-6 md:p-8 font-inter">
  <h1 class="text-3xl sm:text-4xl font-bold text-gray-800 mb-6 sm:mb-8 text-center">API Dashboard</h1>

  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 sm:gap-8">
    <!-- Left Column: Create New API Key -->
    <div class="lg:col-span-1 bg-white rounded-xl shadow-lg p-6 flex flex-col items-center justify-center">
      <CreateApiKey />
    </div>

    <!-- Right Column Top: Metrics and Logs -->
    <div class="lg:col-span-2 bg-white rounded-xl shadow-lg p-6">
      <MetricsAndLogs />
    </div>

    <!-- Full Width Bottom: Table of Active and Inactive APIs -->
    <div class="lg:col-span-3 bg-white rounded-xl shadow-lg p-6">
      <ApiKeysTable />
    </div>
  </div>
</div>

<style>
  /* You can add specific styles here if needed, but Tailwind handles most of it */
</style>