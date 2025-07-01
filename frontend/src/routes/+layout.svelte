<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  
  // You can add global logic here, like authentication checks
  let isClient = false;
  
  onMount(() => {
    isClient = true;
  });

  // Check if current page needs authentication
  $: isAuthPage = $page.route.id?.includes('login') || $page.route.id?.includes('register');
  $: isDashboard = $page.route.id?.includes('dashboard');
</script>

<svelte:head>
  <!-- Global meta tags -->
  <meta name="description" content="API Meter - Track, Manage, Scale Your APIs" />
  <meta name="keywords" content="API monitoring, API analytics, API keys, rate limiting" />
  
  <!-- Open Graph tags -->
  <meta property="og:title" content="API Meter" />
  <meta property="og:description" content="Track, Manage, Scale Your APIs" />
  <meta property="og:type" content="website" />
  
  <!-- Twitter Card tags -->
  <meta name="twitter:card" content="summary_large_image" />
  <meta name="twitter:title" content="API Meter" />
  <meta name="twitter:description" content="Track, Manage, Scale Your APIs" />
</svelte:head>

<!-- Main content slot - this is where page content gets rendered -->
<main class="min-h-screen">
  <slot />
</main>

<!-- Global styles if needed -->
<style>
  :global(html) {
    scroll-behavior: smooth;
  }
  
  :global(body) {
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }
  
  /* Custom scrollbar */
  :global(::-webkit-scrollbar) {
    width: 8px;
  }
  
  :global(::-webkit-scrollbar-track) {
    background: #f1f1f1;
  }
  
  :global(::-webkit-scrollbar-thumb) {
    background: #c1c1c1;
    border-radius: 4px;
  }
  
  :global(::-webkit-scrollbar-thumb:hover) {
    background: #a8a8a8;
  }
</style>