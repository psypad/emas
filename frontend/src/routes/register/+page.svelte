<script lang="ts">
  import { goto } from '$app/navigation';
  
  let name: string = '';
  let email: string = '';
  let password: string = '';
  let confirmPassword: string = '';
  let isLoading: boolean = false;
  let error: string = '';
  let success: string = '';

  interface RegisterRequest {
    name: string;
    email: string;
    password: string;
  }

  interface RegisterResponse {
    token: string;
    user: {
      id: string;
      name: string;
      email: string;
    };
    message?: string;
  }

  interface ErrorResponse {
    message: string;
    errors?: Record<string, string[]>;
  }

  // Form validation
  $: isFormValid = name.length >= 2 && 
                   email.includes('@') && 
                   password.length >= 8 && 
                   password === confirmPassword;

  $: passwordsMatch = password === confirmPassword || confirmPassword === '';

  async function handleRegister(): Promise<void> {
    if (!isFormValid) {
      error = 'Please fill in all fields correctly';
      return;
    }

    if (password !== confirmPassword) {
      error = 'Passwords do not match';
      return;
    }

    isLoading = true;
    error = '';
    success = '';

    try {
      const registerData: RegisterRequest = {
        name,
        email,
        password
      };

      // TODO: Replace with actual API call
      const response = await fetch('http://localhost:3000/api/auth/register', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(registerData)
      });


      if (response.ok) {
        const data: RegisterResponse = await response.json();
        
        // Store token
        localStorage.setItem('token', data.token);
        
        // Show success message briefly
        success = 'Account created successfully! Redirecting...';
        
        // Redirect to dashboard after short delay
        setTimeout(() => {
          goto('/dashboard');
        }, 1500);
      } else {
        const errorData: ErrorResponse = await response.json();
        error = errorData.message || 'Registration failed';
      }
    } catch (err) {
      error = 'Network error. Please try again.';
    } finally {
      isLoading = false;
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === 'Enter' && isFormValid) {
      handleRegister();
    }
  }

  function clearError(): void {
    error = '';
  }
</script>

<svelte:head>
  <title>Create Account - API Meter</title>
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
      Create your account
    </h2>
    <p class="mt-2 text-center text-sm text-gray-600">
      Already have an account?
      <a href="/login" class="font-medium text-indigo-600 hover:text-indigo-500 transition-colors">
        Sign in here
      </a>
    </p>
  </div>

  <!-- Registration Form -->
  <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
    <div class="bg-white py-8 px-4 shadow-lg sm:rounded-lg sm:px-10">
      <form on:submit|preventDefault={handleRegister} class="space-y-6">
        
        <!-- Name Field -->
        <div>
          <label for="name" class="block text-sm font-medium text-gray-700">
            Full Name
          </label>
          <div class="mt-1">
            <input
              id="name"
              name="name"
              type="text"
              autocomplete="name"
              required
              bind:value={name}
              on:input={clearError}
              on:keydown={handleKeydown}
              class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 transition-colors sm:text-sm"
              placeholder="Enter your full name"
            />
          </div>
        </div>

        <!-- Email Field -->
        <div>
          <label for="email" class="block text-sm font-medium text-gray-700">
            Email Address
          </label>
          <div class="mt-1">
            <input
              id="email"
              name="email"
              type="email"
              autocomplete="email"
              required
              bind:value={email}
              on:input={clearError}
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
              autocomplete="new-password"
              required
              bind:value={password}
              on:input={clearError}
              on:keydown={handleKeydown}
              class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 transition-colors sm:text-sm"
              placeholder="Create a password (min 8 characters)"
            />
          </div>
          <p class="mt-1 text-xs text-gray-500">
            Password must be at least 8 characters long
          </p>
        </div>

        <!-- Confirm Password Field -->
        <div>
          <label for="confirmPassword" class="block text-sm font-medium text-gray-700">
            Confirm Password
          </label>
          <div class="mt-1">
            <input
              id="confirmPassword"
              name="confirmPassword"
              type="password"
              autocomplete="new-password"
              required
              bind:value={confirmPassword}
              on:input={clearError}
              on:keydown={handleKeydown}
              class="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md placeholder-gray-400 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 transition-colors sm:text-sm"
              class:border-red-300={confirmPassword && !passwordsMatch}
              class:border-green-300={confirmPassword && passwordsMatch}
              placeholder="Confirm your password"
            />
          </div>
          {#if confirmPassword && !passwordsMatch}
            <p class="mt-1 text-xs text-red-600">
              <i class="fas fa-exclamation-circle mr-1"></i>
              Passwords do not match
            </p>
          {/if}
          {#if confirmPassword && passwordsMatch}
            <p class="mt-1 text-xs text-green-600">
              <i class="fas fa-check-circle mr-1"></i>
              Passwords match
            </p>
          {/if}
        </div>

        <!-- Terms and Conditions -->
        <div class="flex items-center">
          <input
            id="terms"
            name="terms"
            type="checkbox"
            required
            class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
          />
          <label for="terms" class="ml-2 block text-sm text-gray-900">
            I agree to the
            <a href="/terms" class="text-indigo-600 hover:text-indigo-500 transition-colors">Terms of Service</a>
            and
            <a href="/privacy" class="text-indigo-600 hover:text-indigo-500 transition-colors">Privacy Policy</a>
          </label>
        </div>

        <!-- Success Message -->
        {#if success}
          <div class="bg-green-50 border border-green-200 rounded-md p-3">
            <div class="flex">
              <i class="fas fa-check-circle text-green-400 mr-2 mt-0.5"></i>
              <p class="text-sm text-green-600">{success}</p>
            </div>
          </div>
        {/if}

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
            disabled={isLoading || !isFormValid}
            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {#if isLoading}
              <i class="fas fa-spinner fa-spin mr-2"></i>
              Creating Account...
            {:else}
              <i class="fas fa-user-plus mr-2"></i>
              Create Account
            {/if}
          </button>
        </div>
      </form>

      <!-- Social Registration (Optional) -->
      <div class="mt-6">
        <div class="relative">
          <div class="absolute inset-0 flex items-center">
            <div class="w-full border-t border-gray-300"></div>
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

      <!-- Features Preview -->
      <div class="mt-8 pt-6 border-t border-gray-200">
        <h3 class="text-sm font-medium text-gray-900 mb-3">What you'll get:</h3>
        <ul class="text-sm text-gray-600 space-y-1">
          <li class="flex items-center">
            <i class="fas fa-check text-green-500 mr-2"></i>
            Unlimited API key management
          </li>
          <li class="flex items-center">
            <i class="fas fa-check text-green-500 mr-2"></i>
            Real-time usage analytics
          </li>
          <li class="flex items-center">
            <i class="fas fa-check text-green-500 mr-2"></i>
            Advanced rate limiting
          </li>
        </ul>
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