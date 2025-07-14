<script lang="ts">
  import { goto } from '$app/navigation';

  let name: string = '';
  let email: string = '';
  let password: string = '';
  let confirmPassword: string = '';
  let terms: boolean = false;
  let isLoading: boolean = false;
  let error: string = '';
  let success: string = '';

  interface RegisterResponse {
    message: string;
  }

  interface ErrorResponse {
    error: string;
  }

  async function handleRegister(event: Event): Promise<void> {
    event.preventDefault();
    if (!name || !email.includes('@') || password.length < 8 || password !== confirmPassword || !terms) {
      error =
        'Please fill all fields, ensure passwords match, email is valid, password is 8+ characters, and agree to terms.';
      return;
    }

    isLoading = true;
    error = '';
    success = '';

    try {
      const response = await fetch('http://localhost:3000/api/auth/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, email, password }),
      });

      if (response.ok) {
        const data: RegisterResponse = await response.json();
        success = data.message;
        goto('/login');
      } else {
        const errorData: ErrorResponse = await response.json();
        error = errorData.error || 'Registration failed';
      }
    } catch (err) {
      error = 'Network error. Please try again.';
    } finally {
      isLoading = false;
    }
  }
</script>

<fieldset class="field">
  <legend>Register</legend>
  <form on:submit={handleRegister}>
    <div class="form-example">
      <label for="name">Name:</label>
      <input
        bind:value={name}
        type="text"
        name="name"
        id="name"
        required
        placeholder="Enter your name"
      />
    </div>
    <div class="form-example">
      <label for="email">Email:</label>
      <input
        bind:value={email}
        type="email"
        name="email"
        id="email"
        required
        placeholder="Enter your email"
      />
    </div>
    <div class="form-example">
      <label for="password">Password:</label>
      <input
        bind:value={password}
        type="password"
        name="password"
        id="password"
        required
        placeholder="Enter your password"
      />
    </div>
    <div class="form-example">
      <label for="confirm-password">Confirm Password:</label>
      <input
        bind:value={confirmPassword}
        type="password"
        name="confirm-password"
        id="confirm-password"
        required
        placeholder="Confirm your password"
      />
    </div>
    <div class="form-example">
      <input
        bind:checked={terms}
        type="checkbox"
        id="terms"
        name="terms"
      />
      <label for="terms">
        I agree to the <a href="/terms">Terms and Conditions</a>
      </label>
    </div>
    <div class="submit">
      <button type="submit" disabled={isLoading}>
        {isLoading ? 'Registering...' : 'Register'}
      </button>
    </div>
  </form>

  {#if error}
    <p style="color: red;">{error}</p>
  {/if}
  {#if success}
    <p style="color: green;">{success}</p>
  {/if}
</fieldset>

<style>
  .field {
    max-width: 400px;
    margin: 0 auto;
    padding: 20px;
    border: 1px solid #ccc;
    border-radius: 8px;
  }
  .form-example {
    padding: 10px;
    display: flex;
    flex-direction: column;
    gap: 5px;
  }
  .submit {
    padding: 15px;
    text-align: center;
  }
  label {
    font-weight: bold;
  }
  input[type="text"],
  input[type="email"],
  input[type="password"] {
    padding: 8px;
    border: 1px solid #ccc;
    border-radius: 4px;
  }
  button {
    padding: 10px;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  button:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
  }
  button:hover:not(:disabled) {
    background-color: #0056b3;
  }
</style>