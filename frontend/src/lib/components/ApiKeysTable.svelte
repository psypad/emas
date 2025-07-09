<script lang="ts">
  import { onMount } from 'svelte';

  interface ApiKey {
    id: string;
    name: string;
    key_value: string; // In a real app, this would be masked or not returned
    created_at: string;
    is_active: boolean;
  }

  let apiKeys: ApiKey[] = [];
  let isLoading: boolean = true;
  let errorMessage: string | null = null;
  let successMessage: string | null = null;

  onMount(async () => {
    await fetchApiKeys();
  });

  /**
   * Fetches the list of API keys from the backend.
   * This is a dummy implementation; replace with actual API call.
   */
  async function fetchApiKeys() {
    isLoading = true;
    errorMessage = null;
    try {
      // TODO: Replace with actual API call to fetch API keys
      const response = await fetch('http://localhost:3000/api/user/keys', {
        headers: {
          'Authorization': 'Bearer YOUR_AUTH_TOKEN' // Placeholder for authentication
        }
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Failed to fetch API keys.');
      }

      const data = await response.json();
      // Dummy data if API call returns empty or fails
      apiKeys = data.keys || [
        { id: 'key_abc123', name: 'Main App Key', key_value: 'sk_live_abc123...', created_at: '2023-01-15T10:00:00Z', is_active: true },
        { id: 'key_def456', name: 'Dev Environment', key_value: 'sk_test_def456...', created_at: '2023-03-20T14:30:00Z', is_active: false },
        { id: 'key_ghi789', name: 'Analytics Service', key_value: 'sk_live_ghi789...', created_at: '2024-06-01T08:15:00Z', is_active: true },
      ];
    } catch (error: any) {
      errorMessage = error.message;
      console.error('Error fetching API keys:', error);
    } finally {
      isLoading = false;
    }
  }

  /**
   * Toggles the active status of an API key.
   * This is a dummy implementation; replace with actual API call.
   * @param keyId The ID of the API key to toggle.
   * @param currentStatus The current active status of the key.
   */
  async function toggleKeyStatus(keyId: string, currentStatus: boolean) {
    errorMessage = null;
    successMessage = null;

    try {
      // TODO: Replace with actual API call to update API key status
      const response = await fetch(`http://localhost:3000/api/user/keys/${keyId}/status`, {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': 'Bearer YOUR_AUTH_TOKEN' // Placeholder
        },
        body: JSON.stringify({ is_active: !currentStatus })
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.message || 'Failed to update API key status.');
      }

      // Update the local state to reflect the change immediately
      apiKeys = apiKeys.map(key =>
        key.id === keyId ? { ...key, is_active: !currentStatus } : key
      );
      successMessage = `API Key status updated to ${!currentStatus ? 'Active' : 'Inactive'}.`;
    } catch (error: any) {
      errorMessage = error.message;
      console.error('Error toggling API key status:', error);
    }
  }

  /**
   * Masks the API key value for display purposes.
   * In a real application, the full key value should never be returned from the backend
   * unless explicitly requested and secured (e.g., for a one-time reveal).
   * @param key The full API key string.
   * @returns A masked version of the key.
   */
  function maskKey(key: string): string {
    if (!key || key.length < 8) return '********';
    return key.substring(0, 4) + '...' + key.substring(key.length - 4);
  }
</script>

<div class="w-full p-4 bg-white rounded-lg">
  <h2 class="text-2xl font-semibold text-gray-800 mb-4">Your API Keys</h2>

  {#if isLoading}
    <p class="text-gray-600">Loading API keys...</p>
  {:else if errorMessage}
    <p class="text-red-600 text-center">{errorMessage}</p>
  {:else}
    {#if apiKeys.length === 0}
      <p class="text-gray-600 text-center">No API keys found. Generate one to get started!</p>
    {:else}
      <div class="overflow-x-auto rounded-lg border border-gray-200 shadow-sm">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Name
              </th>
              <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Key Value
              </th>
              <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Created At
              </th>
              <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Status
              </th>
              <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Actions
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            {#each apiKeys as key (key.id)}
              <tr>
                <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                  {key.name}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500 font-mono">
                  {maskKey(key.key_value)}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {new Date(key.created_at).toLocaleDateString()}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm">
                  <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full"
                    class:bg-green-100={key.is_active}
                    class:text-green-800={key.is_active}
                    class:bg-red-100={!key.is_active}
                    class:text-red-800={!key.is_active}
                  >
                    {key.is_active ? 'Active' : 'Inactive'}
                  </span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-left text-sm font-medium">
                  <button
                    on:click={() => toggleKeyStatus(key.id, key.is_active)}
                    class="px-4 py-2 rounded-lg text-white font-semibold text-sm transition duration-200 ease-in-out"
                    class:bg-yellow-500={key.is_active}
                    class:hover:bg-yellow-600={key.is_active}
                    class:bg-blue-500={!key.is_active}
                    class:hover:bg-blue-600={!key.is_active}
                  >
                    {key.is_active ? 'Deactivate' : 'Activate'}
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  {/if}

  {#if successMessage}
    <p class="mt-4 text-green-600 text-center text-sm">{successMessage}</p>
  {/if}
</div>