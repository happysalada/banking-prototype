/**
 * Can be made globally available by placing this
 * inside `global.d.ts` and removing `export` keyword
 */

export type FlashType = "ERROR" | "SUCCESS";

export interface Transaction {
  id: string;
  fromId: string;
  toId: string;
  amount: number;
  note: string;
  insertedAt: string;
}

export interface User {
  id: string;
  name: string;
  email: string;
}

