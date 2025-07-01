// place files you want to import through the `$lib` alias in this folder.
// src/lib/types.ts
export interface User {
  id: string;
  email: string;
  name?: string;
  created_at: string;
}

export interface ApiKey {
  id: string;
  name: string;
  key: string;
  user_id: string;
  usage_count: number;
  created_at: string;
  last_used?: string;
}

export interface LoginResponse {
  token: string;
  user: User;
}

export interface ErrorResponse {
  message: string;
  code?: string;
}