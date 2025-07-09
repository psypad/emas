<script lang="ts">
  import { onMount } from 'svelte';

  let newKeyName: string = '';
  let generatedKey: string | null = null;
  let isLoading: boolean = false;
  let errorMessage: string | null = null;
  let successMessage: string | null = null;

  /**
   * Handles the generation of a new API key.
   * This is a dummy implementation; replace with actual API call.
   */
  async function generateApiKey() {
    isLoading = true;
    errorMessage = null;
    successMessage = null;
    generatedKey = null;

    try {
      // TODO: Replace with actual API call to generate a new API key
      const response = await fetch('http://localhost:3000/api/user/keys', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer YOUR_AUTH_TOKEN' // Placeholder for authentication
        },
        body: JSON.stringify({ name: newKeyName || 'New API Key' })
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Failed to generate API key.');
      }

      const data = await response.json();
      generatedKey = data.key_value || 'sk_dummy_generated_key_1234567890'; // Dummy key
      successMessage = 'API Key generated successfully!';
      newKeyName = ''; // Clear input
    } catch (error: any) {
      errorMessage = error.message;
      console.error('Error generating API key:', error);
    } finally {
      isLoading = false;
    }
  }

  /**
   * Copies the generated API key to the clipboard.
   */
  function copyToClipboard() {
    if (generatedKey) {
      // Using execCommand for broader compatibility within iframes
      const el = document.createElement('textarea');
      el.value = generatedKey;
      document.body.appendChild(el);
      el.select();
      document.execCommand('copy');
      document.body.removeChild(el);
      successMessage = 'API Key copied to clipboard!';
    }
  }
</script>

<div class="w-full max-w-md p-4 bg-white rounded-lg">
  <h2 class="text-2xl font-semibold text-gray-800 mb-4 text-center">Generate New API Key</h2>

  <div class="mb-4">
    <label for="keyName" class="block text-gray-700 text-sm font-bold mb-2">Key Name (Optional):</label>
    <input
      type="text"
      id="keyName"
      bind:value={newKeyName}
      placeholder="e.g., My Project Key"
      class="shadow appearance-none border rounded-lg w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
    />
  </div>

  <button
    on:click={generateApiKey}
    disabled={isLoading}
    class="w-full bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 transition duration-300 ease-in-out disabled:opacity-50 disabled:cursor-not-allowed"
  >
    {#if isLoading}
      Generating...
    {:else}
      Generate Key
    {/if}
  </button>

  {#if generatedKey}
    <div class="mt-6 p-4 bg-blue-50 border border-blue-200 rounded-lg text-sm text-blue-800">
      <p class="font-semibold mb-2">Your New API Key:</p>
      <div class="flex items-center space-x-2">
        <input
          type="text"
          value={generatedKey}
          readonly
          class="flex-grow bg-blue-100 border border-blue-300 rounded-md py-1 px-2 text-blue-900 font-mono text-xs sm:text-sm"
        />
        <button
          on:click={copyToClipboard}
          class="bg-blue-200 hover:bg-blue-300 text-blue-800 font-semibold py-1 px-3 rounded-md text-xs sm:text-sm transition duration-200 ease-in-out"
        >
          Copy
        </button>
      </div>
      <p class="mt-2 text-xs text-blue-600">Please save this key securely. It will not be shown again.</p>
    </div>
  {/if}

  {#if successMessage}
    <p class="mt-4 text-green-600 text-center text-sm">{successMessage}</p>
  {/if}

  {#if errorMessage}
    <p class="mt-4 text-red-600 text-center text-sm">{errorMessage}</p>
  {/if}
</div>