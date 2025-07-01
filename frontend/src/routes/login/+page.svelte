<script lang="ts">
  import { goto } from '$app/navigation';
  
  let email: string = '';
  let password: string = '';
  let isLoading: boolean = false;
  let error: string = '';

  interface LoginResponse {
    token: string;
    message?: string;
  }

  interface ErrorResponse {
    message: string;
  }

  async function handleLogin(): Promise<void> {
    if (!email || !password) {
      error = 'Please fill in all fields';
      return;
    }

    isLoading = true;
    error = '';

    try {
      // TODO: Replace with actual API call
      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ email, password })
      });

      if (response.ok) {
        const data: LoginResponse = await response.json();
        // Store token (you might want to use a store for this)
        localStorage.setItem('token', data.token);
        goto('/dashboard');
      } else {
        const errorData: ErrorResponse = await response.json();
        error = errorData.message || 'Login failed';
      }
    } catch (err) {
      error = 'Network error. Please try again.';
    } finally {
      isLoading = false;
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === 'Enter') {
      handleLogin();
    }
  }
</script>

<svelte:head>
  <title>Login - API Meter</title>
  <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css" rel="stylesheet">
</svelte:head>

<div class="min-h-screen bg-gray-50 flex flex-col justify-center py-12 sm:px-6 lg:px-8">
  <!-- Header with Logo -->
  <div class="sm:mx-auto sm:w-full sm:max-w-md">
    <div class="flex justify-center items-center mb-6">
      <i class="fas fa-chart-line text-3xl text-indigo-600 mr-3"></i>
      <span class="text-2xl font-bold text-gray-900">API Meter</span>
    </div>
    <h2 class="text-center text-3xl font-extrabold text-gray-900">
      Sign in to your account
    </h2>
    <p class="mt-2 text-center text-sm text-gray-600">
      Or
      <a href="/register" class="font-medium text-indigo-600 hover:text-indigo-500 transition-colors">
        create a new account
      </a>
    </p>
  </div>

  <!-- Login Form -->
  <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
    <div class="bg-white py-8 px-4 shadow-lg sm:rounded-lg sm:px-10">
      <form on:submit|preventDefault={handleLogin} class="space-y-6">
        <!-- Email Field -->
        <div>
          <label for="email" class="block text-sm font-medium text-gray-700">
            Email address
          </label>
          <div class="mt-1">
            <input
              id="email"
              name="email"
              type="email"
              autocomplete="email"
              required
              bind:value={email}
              on:keydown={handleKeydown}
              class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 transition-colors sm:text-sm"
              placeholder="Enter your email"
            />
          </div>
        </div>

        <!-- Password Field -->
        <div>
          <label for="password" class="block text-sm font-medium text-gray-700">
            Password
          </label>
          <div class="mt-1">
            <input
              id="password"
              name="password"
              type="password"
              autocomplete="current-password"
              required
              bind:value={password}
              on:keydown={handleKeydown}
              class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 transition-colors sm:text-sm"
              placeholder="Enter your password"
            />
          </div>
        </div>

        <!-- Remember Me & Forgot Password -->
        <div class="flex items-center justify-between">
          <div class="flex items-center">
            <input
              id="remember-me"
              name="remember-me"
              type="checkbox"
              class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
            />
            <label for="remember-me" class="ml-2 block text-sm text-gray-900">
              Remember me
            </label>
          </div>

          <div class="text-sm">
            <a href="/forgot-password" class="font-medium text-indigo-600 hover:text-indigo-500 transition-colors">
              Forgot your password?
            </a>
          </div>
        </div>

        <!-- Error Message -->
        {#if error}
          <div class="bg-red-50 border border-red-200 rounded-md p-3">
            <div class="flex">
              <i class="fas fa-exclamation-circle text-red-400 mr-2 mt-0.5"></i>
              <p class="text-sm text-red-600">{error}</p>
            </div>
          </div>
        {/if}

        <!-- Submit Button -->
        <div>
          <button
            type="submit"
            disabled={isLoading}
            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {#if isLoading}
              <i class="fas fa-spinner fa-spin mr-2"></i>
              Signing in...
            {:else}
              <i class="fas fa-sign-in-alt mr-2"></i>
              Sign in
            {/if}
          </button>
        </div>
      </form>

      <!-- Social Login (Optional) -->
      <div class="mt-6">
        <div class="relative">
          <div class="absolute inset-0 flex items-center">
            <div class="w-full border-t border-gray-300" />
          </div>
          <div class="relative flex justify-center text-sm">
            <span class="px-2 bg-white text-gray-500">Or continue with</span>
          </div>
        </div>

        <div class="mt-6 grid grid-cols-2 gap-3">
          <button
            type="button"
            class="w-full inline-flex justify-center py-2 px-4 border border-gray-300 rounded-md shadow-sm bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 transition-colors"
          >
            <i class="fab fa-google text-red-500 mr-2"></i>
            Google
          </button>

          <button
            type="button"
            class="w-full inline-flex justify-center py-2 px-4 border border-gray-300 rounded-md shadow-sm bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 transition-colors"
          >
            <i class="fab fa-github mr-2"></i>
            GitHub
          </button>
        </div>
      </div>
    </div>
  </div>

  <!-- Back to Home Link -->
  <div class="mt-8 text-center">
    <a href="/" class="text-indigo-600 hover:text-indigo-500 transition-colors">
      <i class="fas fa-arrow-left mr-1"></i>
      Back to home
    </a>
  </div>
</div>