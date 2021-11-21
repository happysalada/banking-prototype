<script context="module" lang="ts">
  import { query } from "$lib/api";

  // see https://kit.svelte.dev/docs#loading
  export async function load({ page, fetch }) {
    const id = page.params.id;
    const response = await query( fetch,
      `{
        user(userId: "${id}") { id, name, email } 
        transactions(userId: "${id}") { id, fromId, toId, amount, note, insertedAt }
      }`
    );

    if (response.ok) {
      const { data, errors } = await response.json();
      if (errors && errors.length > 0) {
        const flashMessage = errors
          .map(({ message }) => message.toString())
          .join("\n");
        console.error(flashMessage);
        return {
          props: { flashMessage },
        };
      } else {
        const { user, transactions } = data;

        return {
          props: { user, transactions },
        };
      }
    } else {
      const { message } = await response.json();
      return {
        props: {
          flashMessage: message,
        },
      };
    }
  }
</script>

<script lang="ts">
  import type { Transaction, User } from "$lib/types";
  export let user: User;
  export let transactions: Transaction[];
</script>

<div class="min-h-full">
  <!-- Off-canvas menu for mobile, show/hide based on off-canvas menu state. -->
  <div class="fixed inset-0 flex z-40 lg:hidden" role="dialog" aria-modal="true">
    <!--
      Off-canvas menu overlay, show/hide based on off-canvas menu state.

      Entering: "transition-opacity ease-linear duration-300"
        From: "opacity-0"
        To: "opacity-100"
      Leaving: "transition-opacity ease-linear duration-300"
        From: "opacity-100"
        To: "opacity-0"
    -->
    <div class="fixed inset-0 bg-gray-600 bg-opacity-75" aria-hidden="true"></div>

    <!--
      Off-canvas menu, show/hide based on off-canvas menu state.

      Entering: "transition ease-in-out duration-300 transform"
        From: "-translate-x-full"
        To: "translate-x-0"
      Leaving: "transition ease-in-out duration-300 transform"
        From: "translate-x-0"
        To: "-translate-x-full"
    -->
    <div class="relative flex-1 flex flex-col max-w-xs w-full pt-5 pb-4 bg-cyan-700">
      <!--
        Close button, show/hide based on off-canvas menu state.

        Entering: "ease-in-out duration-300"
          From: "opacity-0"
          To: "opacity-100"
        Leaving: "ease-in-out duration-300"
          From: "opacity-100"
          To: "opacity-0"
      -->
      <div class="absolute top-0 right-0 -mr-12 pt-2">
        <button type="button" class="ml-1 flex items-center justify-center h-10 w-10 rounded-full focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white">
          <span class="sr-only">Close sidebar</span>
          <!-- Heroicon name: outline/x -->
          <svg class="h-6 w-6 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>

      <div class="flex-shrink-0 flex items-center px-4">
        <img class="h-8 w-auto" src="data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTg2IiBoZWlnaHQ9IjMwIiB2aWV3Qm94PSIwIDAgMTg2IDMwIiBmaWxsPSJub25lIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgo8cGF0aCBkPSJNMTcuODE0IDFDNy45NzUyNCAxIDAgOC45Nzk0MSAwIDE4LjgyMjNWMjYuNDU5OUMwIDI3Ljg2NDggMS4xMzgxMyAyOS4wMDMgMi41NDMwNyAyOS4wMDNDMy45NDgwMiAyOS4wMDMgNS4wOTQ0OCAyNy44NjQ4IDUuMDk0NDggMjYuNDU5OVYxOC44MjIzQzUuMDk0NDggMTcuNDEzMiA2LjIyODQ0IDE2LjI3NTEgNy42Mzc1NSAxNi4yNzUxQzkuMDQyNSAxNi4yNzUxIDEwLjE4MDYgMTcuNDEzMiAxMC4xODA2IDE4LjgyMjNWMjYuNDU5OUMxMC4xODA2IDI3Ljg2NDggMTEuMzE4OCAyOS4wMDMgMTIuNzIzNyAyOS4wMDNDMTQuMTMyOCAyOS4wMDMgMTUuMjcwOSAyNy44NjQ4IDE1LjI3MDkgMjYuNDU5OVYxOC44MjIzQzE1LjI3MDkgMTcuNDEzMiAxNi40MDkxIDE2LjI3NTEgMTcuODE0IDE2LjI3NTFDMTkuMjIzMSAxNi4yNzUxIDIwLjM2NTQgMTcuNDEzMiAyMC4zNjU0IDE4LjgyMjNWMjYuNDU5OUMyMC4zNjU0IDI3Ljg2NDggMjEuNTAzNiAyOS4wMDMgMjIuOTA4NSAyOS4wMDNDMjQuMzEzNCAyOS4wMDMgMjUuNDUxNiAyNy44NjQ4IDI1LjQ1MTYgMjYuNDU5OVYxOC44MjIzQzI1LjQ1MTYgMTcuNDEzMiAyNi41ODk3IDE2LjI3NTEgMjguMDAzIDE2LjI3NTFDMjkuNDA3OSAxNi4yNzUxIDMwLjU0NiAxNy40MTMyIDMwLjU0NiAxOC44MjIzVjI2LjQ1OTlDMzAuNTQ2IDI3Ljg2NDggMzEuNjg0MiAyOS4wMDMgMzMuMDkzMyAyOS4wMDNDMzQuNDk4MiAyOS4wMDMgMzUuNjM2NCAyNy44NjQ4IDM1LjYzNjQgMjYuNDU5OVYxOC44MjIzQzM1LjYzNjQgOC45Nzk0MSAyNy42NTcgMSAxNy44MTQgMVoiIGZpbGw9IndoaXRlIi8+CjxwYXRoIGQ9Ik0zOS42MzY0IDEuMDAyNzJINDQuMTU3OFYxMi44OTI2TDU2LjIyMzUgMS4wMDI3Mkg2Mi4yMzU0TDUxLjQ3NiAxMS40MTA1TDYyLjgzODMgMjguOTk0MUg1Ni44MjY0TDQ4LjAyMiAxNC43NTE0TDQ0LjE1NzggMTguNDlWMjguOTk4M0gzOS42MzY0VjEuMDAyNzJaIiBmaWxsPSJ3aGl0ZSIvPgo8cGF0aCBkPSJNNjUuMjk1OCAxLjAwMzA1SDY5Ljc2NzFWNS44Mzg1M0M3MC4xMzEzIDQuODk2NTYgNzEuMDMxNCAzLjc1MzYyIDcyLjQ1OSAyLjQwMTM2QzczLjg4NjcgMS4wNTMyOSA3NS41MzYyIDAuMzc1MDY2IDc3LjM5OTIgMC4zNzUwNjZDNzcuNDg3MSAwLjM3NTA2NiA3Ny42MzM2IDAuMzgzNDM5IDc3Ljg0MyAwLjQwMDE4NkM3OC4wNTIzIDAuNDE2OTMyIDc4LjQwODEgMC40NTA0MjUgNzguOTE0NyAwLjUwNDg1VjUuNDc0M0M3OC42MzQyIDUuNDE5ODggNzguMzc4OCA1LjM4NjM4IDc4LjE0NDQgNS4zNjk2NEM3Ny45MDk5IDUuMzUyODkgNzcuNjUwNCA1LjM0NDUyIDc3LjM3NDEgNS4zNDQ1MkM3NS4wMDQ1IDUuMzQ0NTIgNzMuMTgzMyA2LjEwNjQ3IDcxLjkxMDYgNy42MzAzOEM3MC42Mzc5IDkuMTU0MjkgNzAuMDAxNSAxMC45MDg1IDcwLjAwMTUgMTIuODk3MVYyOC45OTg2SDY1LjI5NThWMS4wMDMwNVoiIGZpbGw9IndoaXRlIi8+CjxwYXRoIGQ9Ik05Ni40Mzk3IDEyLjA1NTZDOTcuNTE5OCAxMS45MTc0IDk4LjI0NDEgMTEuNDY1MyA5OC42MDgzIDEwLjY5OTFDOTguODE3NiAxMC4yODA1IDk4LjkyMjMgOS42Nzc2MSA5OC45MjIzIDguODkwNTRDOTguOTIyMyA3LjI4MjkgOTguMzUyOSA2LjExNDg1IDk3LjIxNDIgNS4zODYzOUM5Ni4wNzU0IDQuNjYyMTEgOTQuNDQ2OSA0LjI5Nzg4IDkyLjMyNDMgNC4yOTc4OEM4OS44NzEgNC4yOTc4OCA4OC4xMzM1IDQuOTYzNTQgODcuMTA3OCA2LjI5NDg3Qzg2LjUzNDMgNy4wMzE3MSA4Ni4xNjE3IDguMTI0NCA4NS45ODU4IDkuNTc3MTRIODEuNTk0MUM4MS42ODIgNi4xMTkwMyA4Mi43OTk4IDMuNzExNzYgODQuOTQ3NiAyLjM1NTMxQzg3LjA5OTQgMS4wMDMwNSA4OS41OTA1IDAuMzI0ODI5IDkyLjQyOSAwLjMyNDgyOUM5NS43MTk2IDAuMzI0ODI5IDk4LjM5MDYgMC45NTI4MTQgMTAwLjQ0MiAyLjIwNDZDMTAyLjQ4MSAzLjQ2MDU3IDEwMy40OTggNS40MTE1IDEwMy40OTggOC4wNjE2VjI0LjE4ODJDMTAzLjQ5OCAyNC42NzgxIDEwMy41OTkgMjUuMDY3NCAxMDMuOCAyNS4zNjQ3QzEwMy45OTYgMjUuNjYxOSAxMDQuNDE5IDI1LjgwODQgMTA1LjA2OCAyNS44MDg0QzEwNS4yNzggMjUuODA4NCAxMDUuNTEyIDI1Ljc5NTkgMTA1Ljc3NiAyNS43NzA4QzEwNi4wMzUgMjUuNzQ1NiAxMDYuMzE2IDI1LjcwMzggMTA2LjYxMyAyNS42NTM1VjI5LjEyODRDMTA1Ljg4IDI5LjMzNzcgMTA1LjMyNCAyOS40Njc1IDEwNC45NDMgMjkuNTIxOUMxMDQuNTU3IDI5LjU3MjIgMTA0LjAzOCAyOS42MDE1IDEwMy4zNzMgMjkuNjAxNUMxMDEuNzUyIDI5LjYwMTUgMTAwLjU3NiAyOS4wMjc5IDk5Ljg0MzQgMjcuODc2NkM5OS40NjI0IDI3LjI2NTQgOTkuMTkwMyAyNi40MDI5IDk5LjAzNTQgMjUuMjg5M0M5OC4wNzY2IDI2LjU0NTMgOTYuNzAzNCAyNy42MzM4IDk0LjkwNzQgMjguNTU0OEM5My4xMTE0IDI5LjQ4MDEgOTEuMTM1MyAyOS45NDA2IDg4Ljk3NSAyOS45NDA2Qzg2LjM3OTQgMjkuOTQwNiA4NC4yNTY4IDI5LjE1MzUgODIuNjExNSAyNy41Nzk0QzgwLjk2NjEgMjYuMDA1MiA4MC4xNDE0IDI0LjAzMzMgODAuMTQxNCAyMS42Njc5QzgwLjE0MTQgMTkuMDc2NSA4MC45NTM2IDE3LjA2NjkgODIuNTczOCAxNS42MzkzQzg0LjE5NCAxNC4yMTE3IDg2LjMyMDggMTMuMzMyNSA4OC45NDk5IDEzLjAwMTdMOTYuNDM5NyAxMi4wNTU2Wk04Ni41MzQzIDI0Ljc2NkM4Ny41MjY1IDI1LjU0ODkgODguNzAyOSAyNS45MzgyIDkwLjA2MzUgMjUuOTM4MkM5MS43MTcyIDI1LjkzODIgOTMuMzIwNyAyNS41NTczIDk0Ljg3MzkgMjQuNzkxMUM5Ny40ODYzIDIzLjUyMjYgOTguNzkyNSAyMS40NDE5IDk4Ljc5MjUgMTguNTUzMVYxNC43Njg1Qzk4LjIxOSAxNS4xMzY5IDk3LjQ3NzkgMTUuNDQyNSA5Ni41NzM2IDE1LjY4NTNDOTUuNjY5MyAxNS45MjgyIDk0Ljc4MTggMTYuMTA0IDkzLjkxNTIgMTYuMjA4N0w5MS4wNzI1IDE2LjU3MjlDODkuMzY4NiAxNi43OTkgODguMDg3NSAxNy4xNTQ4IDg3LjIyOTIgMTcuNjQwNUM4NS43NzIzIDE4LjQ1NjggODUuMDQ4IDE5Ljc2MzEgODUuMDQ4IDIxLjU1MDdDODUuMDQzOCAyMi45MTU1IDg1LjU0MjEgMjMuOTgzMSA4Ni41MzQzIDI0Ljc2NloiIGZpbGw9IndoaXRlIi8+CjxwYXRoIGQ9Ik0xMTQuMTk1IDEuMDAzMDVWMTIuODkyOUwxMjYuMjYxIDEuMDAzMDVIMTMyLjI2OEwxMjEuNTA5IDExLjQxMDlMMTMyLjg3MSAyOC45OTQ0SDEyNi44NTlMMTE4LjA1NSAxNC43NTE3TDExNC4xOTEgMTguNDkwM1YyOC45OTg2SDEwOS42NjlWMS4wMDMwNUgxMTQuMTk1WiIgZmlsbD0id2hpdGUiLz4KPHBhdGggZD0iTTE1Mi4wNjIgMS43NzMzOEMxNTMuOTE3IDIuNzA2OTggMTU1LjMyOCAzLjkxMjcxIDE1Ni4yOTUgNS4zOTQ3NkMxNTcuMjI5IDYuODA1NjMgMTU3Ljg1MiA4LjQ1MDk1IDE1OC4xNjYgMTAuMzM0OUMxNTguNDQzIDExLjYyNDQgMTU4LjU4MSAxMy42OCAxNTguNTgxIDE2LjUwMTdIMTM4LjIyMkMxMzguMzEgMTkuMzQ4NiAxMzguOTc1IDIxLjYzNDQgMTQwLjIxOSAyMy4zNTUxQzE0MS40NjIgMjUuMDc1OCAxNDMuMzkyIDI1LjkzODIgMTQ2IDI1LjkzODJDMTQ4LjQzNyAyNS45MzgyIDE1MC4zODQgMjUuMTI2IDE1MS44MzYgMjMuNDk3NUMxNTIuNjQ5IDIyLjU1MTMgMTUzLjIyNiAyMS40NTg2IDE1My41NjUgMjAuMjE1MkgxNTguMTkyQzE1OC4wNyAyMS4yNDUxIDE1Ny42NjQgMjIuMzg4IDE1Ni45NzcgMjMuNjUyNEMxNTYuMjkxIDI0LjkxNjcgMTU1LjUyMSAyNS45NDY2IDE1NC42NzUgMjYuNzUwNEMxNTMuMjU2IDI4LjE0NDYgMTUxLjUwMSAyOS4wODY1IDE0OS40MDggMjkuNTcyMkMxNDguMjg2IDI5Ljg1MjcgMTQ3LjAxMyAyOS45OTA4IDE0NS41OTQgMjkuOTkwOEMxNDIuMTMyIDI5Ljk5MDggMTM5LjE5NyAyOC43MjIzIDEzNi43OSAyNi4xODk0QzEzNC4zODMgMjMuNjUyNCAxMzMuMTgxIDIwLjEwNjQgMTMzLjE4MSAxNS41Mzg4QzEzMy4xODEgMTEuMDQyNCAxMzQuMzkxIDcuMzkxNzUgMTM2LjgxOSA0LjU4Njc1QzEzOS4yNDMgMS43ODE3NSAxNDIuNDEyIDAuMzc5MjUzIDE0Ni4zMjcgMC4zNzkyNTNDMTQ4LjI5NCAwLjM3NTA2NyAxNTAuMjA4IDAuODQzOTYyIDE1Mi4wNjIgMS43NzMzOFpNMTUzLjc0NSAxMi43NjczQzE1My41NTcgMTAuNzI4NCAxNTMuMTEzIDkuMDk5ODcgMTUyLjQyMiA3Ljg4MTU4QzE1MS4xNDYgNS42MTY2NSAxNDkuMDEgNC40ODIwOSAxNDYuMDIxIDQuNDgyMDlDMTQzLjg3OCA0LjQ4MjA5IDE0Mi4wODIgNS4yNjA3OSAxNDAuNjMzIDYuODIyMzhDMTM5LjE4IDguMzgzOTcgMTM4LjQxNCAxMC4zNjQyIDEzOC4zMjYgMTIuNzY3M0gxNTMuNzQ1WiIgZmlsbD0id2hpdGUiLz4KPHBhdGggZD0iTTE2Ny4xODQgMS4wMDMwNVY0Ljk3NjFDMTY4LjUwNyAzLjMzOTE1IDE2OS45MSAyLjE2MjczIDE3MS4zOTIgMS40NDY4M0MxNzIuODc0IDAuNzMwOTI0IDE3NC41MTkgMC4zNzUwNjYgMTc2LjMzMiAwLjM3NTA2NkMxODAuMzA1IDAuMzc1MDY2IDE4Mi45ODkgMS43NjA4MiAxODQuMzgzIDQuNTMyMzNDMTg1LjE0OSA2LjA0Nzg2IDE4NS41MzQgOC4yMTY1IDE4NS41MzQgMTEuMDQyNFYyOC45OTg2SDE4MC43NDlWMTEuMzU2NEMxODAuNzQ5IDkuNjQ4MzEgMTgwLjQ5OCA4LjI3MDkzIDE3OS45OTEgNy4yMjg0N0MxNzkuMTU0IDUuNDg2ODYgMTc3LjYzOCA0LjYxNjA2IDE3NS40NDQgNC42MTYwNkMxNzQuMzMxIDQuNjE2MDYgMTczLjQxNCA0LjcyOTA5IDE3Mi42OTggNC45NTUxN0MxNzEuNDA4IDUuMzQwMzMgMTcwLjI3OCA2LjEwNjQ3IDE2OS4yOTggNy4yNTM1OUMxNjguNTE2IDguMTc4ODIgMTY4LjAwNSA5LjEyOTE3IDE2Ny43NyAxMC4xMTcyQzE2Ny41MzYgMTEuMTA1MiAxNjcuNDE5IDEyLjUwNzcgMTY3LjQxOSAxNC4zMzczVjI4Ljk5ODZIMTYyLjcxM1YxLjAwMzA1SDE2Ny4xODRaIiBmaWxsPSJ3aGl0ZSIvPgo8L3N2Zz4K" alt="Easywire logo">
      </div>
      <nav class="mt-5 flex-shrink-0 h-full divide-y divide-cyan-800 overflow-y-auto" aria-label="Sidebar">
        <div class="px-2 space-y-1">
          <!-- Current: "bg-cyan-800 text-white", Default: "text-cyan-100 hover:text-white hover:bg-cyan-600" -->
          <a href="#" class="bg-cyan-800 text-white group flex items-center px-2 py-2 text-base font-medium rounded-md" aria-current="page">
            <!-- Heroicon name: outline/home -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
            </svg>
            Home
          </a>

          <a href="#" class="text-cyan-100 hover:text-white hover:bg-cyan-600 group flex items-center px-2 py-2 text-base font-medium rounded-md">
            <!-- Heroicon name: outline/clock -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            History
          </a>

          <a href="#" class="text-cyan-100 hover:text-white hover:bg-cyan-600 group flex items-center px-2 py-2 text-base font-medium rounded-md">
            <!-- Heroicon name: outline/scale -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3" />
            </svg>
            Balances
          </a>

          <a href="#" class="text-cyan-100 hover:text-white hover:bg-cyan-600 group flex items-center px-2 py-2 text-base font-medium rounded-md">
            <!-- Heroicon name: outline/credit-card -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
            </svg>
            Cards
          </a>

          <a href="#" class="text-cyan-100 hover:text-white hover:bg-cyan-600 group flex items-center px-2 py-2 text-base font-medium rounded-md">
            <!-- Heroicon name: outline/user-group -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
            </svg>
            Recipients
          </a>

          <a href="#" class="text-cyan-100 hover:text-white hover:bg-cyan-600 group flex items-center px-2 py-2 text-base font-medium rounded-md">
            <!-- Heroicon name: outline/document-report -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            Reports
          </a>
        </div>
        <div class="mt-6 pt-6">
          <div class="px-2 space-y-1">
            <a href="#" class="group flex items-center px-2 py-2 text-base font-medium rounded-md text-cyan-100 hover:text-white hover:bg-cyan-600">
              <!-- Heroicon name: outline/cog -->
              <svg class="mr-4 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              Settings
            </a>

            <a href="#" class="group flex items-center px-2 py-2 text-base font-medium rounded-md text-cyan-100 hover:text-white hover:bg-cyan-600">
              <!-- Heroicon name: outline/question-mark-circle -->
              <svg class="mr-4 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              Help
            </a>

            <a href="#" class="group flex items-center px-2 py-2 text-base font-medium rounded-md text-cyan-100 hover:text-white hover:bg-cyan-600">
              <!-- Heroicon name: outline/shield-check -->
              <svg class="mr-4 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
              </svg>
              Privacy
            </a>
          </div>
        </div>
      </nav>
    </div>

    <div class="flex-shrink-0 w-14" aria-hidden="true">
      <!-- Dummy element to force sidebar to shrink to fit close icon -->
    </div>
  </div>

  <!-- Static sidebar for desktop -->
  <div class="hidden lg:flex lg:w-64 lg:flex-col lg:fixed lg:inset-y-0">
    <!-- Sidebar component, swap this element with another sidebar if you like -->
    <div class="flex flex-col flex-grow bg-indigo-400 pt-5 pb-4 overflow-y-auto">
      <div class="flex items-center flex-shrink-0 px-4">
        <img class="h-8 w-auto" src="data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTg2IiBoZWlnaHQ9IjMwIiB2aWV3Qm94PSIwIDAgMTg2IDMwIiBmaWxsPSJub25lIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPgo8cGF0aCBkPSJNMTcuODE0IDFDNy45NzUyNCAxIDAgOC45Nzk0MSAwIDE4LjgyMjNWMjYuNDU5OUMwIDI3Ljg2NDggMS4xMzgxMyAyOS4wMDMgMi41NDMwNyAyOS4wMDNDMy45NDgwMiAyOS4wMDMgNS4wOTQ0OCAyNy44NjQ4IDUuMDk0NDggMjYuNDU5OVYxOC44MjIzQzUuMDk0NDggMTcuNDEzMiA2LjIyODQ0IDE2LjI3NTEgNy42Mzc1NSAxNi4yNzUxQzkuMDQyNSAxNi4yNzUxIDEwLjE4MDYgMTcuNDEzMiAxMC4xODA2IDE4LjgyMjNWMjYuNDU5OUMxMC4xODA2IDI3Ljg2NDggMTEuMzE4OCAyOS4wMDMgMTIuNzIzNyAyOS4wMDNDMTQuMTMyOCAyOS4wMDMgMTUuMjcwOSAyNy44NjQ4IDE1LjI3MDkgMjYuNDU5OVYxOC44MjIzQzE1LjI3MDkgMTcuNDEzMiAxNi40MDkxIDE2LjI3NTEgMTcuODE0IDE2LjI3NTFDMTkuMjIzMSAxNi4yNzUxIDIwLjM2NTQgMTcuNDEzMiAyMC4zNjU0IDE4LjgyMjNWMjYuNDU5OUMyMC4zNjU0IDI3Ljg2NDggMjEuNTAzNiAyOS4wMDMgMjIuOTA4NSAyOS4wMDNDMjQuMzEzNCAyOS4wMDMgMjUuNDUxNiAyNy44NjQ4IDI1LjQ1MTYgMjYuNDU5OVYxOC44MjIzQzI1LjQ1MTYgMTcuNDEzMiAyNi41ODk3IDE2LjI3NTEgMjguMDAzIDE2LjI3NTFDMjkuNDA3OSAxNi4yNzUxIDMwLjU0NiAxNy40MTMyIDMwLjU0NiAxOC44MjIzVjI2LjQ1OTlDMzAuNTQ2IDI3Ljg2NDggMzEuNjg0MiAyOS4wMDMgMzMuMDkzMyAyOS4wMDNDMzQuNDk4MiAyOS4wMDMgMzUuNjM2NCAyNy44NjQ4IDM1LjYzNjQgMjYuNDU5OVYxOC44MjIzQzM1LjYzNjQgOC45Nzk0MSAyNy42NTcgMSAxNy44MTQgMVoiIGZpbGw9IndoaXRlIi8+CjxwYXRoIGQ9Ik0zOS42MzY0IDEuMDAyNzJINDQuMTU3OFYxMi44OTI2TDU2LjIyMzUgMS4wMDI3Mkg2Mi4yMzU0TDUxLjQ3NiAxMS40MTA1TDYyLjgzODMgMjguOTk0MUg1Ni44MjY0TDQ4LjAyMiAxNC43NTE0TDQ0LjE1NzggMTguNDlWMjguOTk4M0gzOS42MzY0VjEuMDAyNzJaIiBmaWxsPSJ3aGl0ZSIvPgo8cGF0aCBkPSJNNjUuMjk1OCAxLjAwMzA1SDY5Ljc2NzFWNS44Mzg1M0M3MC4xMzEzIDQuODk2NTYgNzEuMDMxNCAzLjc1MzYyIDcyLjQ1OSAyLjQwMTM2QzczLjg4NjcgMS4wNTMyOSA3NS41MzYyIDAuMzc1MDY2IDc3LjM5OTIgMC4zNzUwNjZDNzcuNDg3MSAwLjM3NTA2NiA3Ny42MzM2IDAuMzgzNDM5IDc3Ljg0MyAwLjQwMDE4NkM3OC4wNTIzIDAuNDE2OTMyIDc4LjQwODEgMC40NTA0MjUgNzguOTE0NyAwLjUwNDg1VjUuNDc0M0M3OC42MzQyIDUuNDE5ODggNzguMzc4OCA1LjM4NjM4IDc4LjE0NDQgNS4zNjk2NEM3Ny45MDk5IDUuMzUyODkgNzcuNjUwNCA1LjM0NDUyIDc3LjM3NDEgNS4zNDQ1MkM3NS4wMDQ1IDUuMzQ0NTIgNzMuMTgzMyA2LjEwNjQ3IDcxLjkxMDYgNy42MzAzOEM3MC42Mzc5IDkuMTU0MjkgNzAuMDAxNSAxMC45MDg1IDcwLjAwMTUgMTIuODk3MVYyOC45OTg2SDY1LjI5NThWMS4wMDMwNVoiIGZpbGw9IndoaXRlIi8+CjxwYXRoIGQ9Ik05Ni40Mzk3IDEyLjA1NTZDOTcuNTE5OCAxMS45MTc0IDk4LjI0NDEgMTEuNDY1MyA5OC42MDgzIDEwLjY5OTFDOTguODE3NiAxMC4yODA1IDk4LjkyMjMgOS42Nzc2MSA5OC45MjIzIDguODkwNTRDOTguOTIyMyA3LjI4MjkgOTguMzUyOSA2LjExNDg1IDk3LjIxNDIgNS4zODYzOUM5Ni4wNzU0IDQuNjYyMTEgOTQuNDQ2OSA0LjI5Nzg4IDkyLjMyNDMgNC4yOTc4OEM4OS44NzEgNC4yOTc4OCA4OC4xMzM1IDQuOTYzNTQgODcuMTA3OCA2LjI5NDg3Qzg2LjUzNDMgNy4wMzE3MSA4Ni4xNjE3IDguMTI0NCA4NS45ODU4IDkuNTc3MTRIODEuNTk0MUM4MS42ODIgNi4xMTkwMyA4Mi43OTk4IDMuNzExNzYgODQuOTQ3NiAyLjM1NTMxQzg3LjA5OTQgMS4wMDMwNSA4OS41OTA1IDAuMzI0ODI5IDkyLjQyOSAwLjMyNDgyOUM5NS43MTk2IDAuMzI0ODI5IDk4LjM5MDYgMC45NTI4MTQgMTAwLjQ0MiAyLjIwNDZDMTAyLjQ4MSAzLjQ2MDU3IDEwMy40OTggNS40MTE1IDEwMy40OTggOC4wNjE2VjI0LjE4ODJDMTAzLjQ5OCAyNC42NzgxIDEwMy41OTkgMjUuMDY3NCAxMDMuOCAyNS4zNjQ3QzEwMy45OTYgMjUuNjYxOSAxMDQuNDE5IDI1LjgwODQgMTA1LjA2OCAyNS44MDg0QzEwNS4yNzggMjUuODA4NCAxMDUuNTEyIDI1Ljc5NTkgMTA1Ljc3NiAyNS43NzA4QzEwNi4wMzUgMjUuNzQ1NiAxMDYuMzE2IDI1LjcwMzggMTA2LjYxMyAyNS42NTM1VjI5LjEyODRDMTA1Ljg4IDI5LjMzNzcgMTA1LjMyNCAyOS40Njc1IDEwNC45NDMgMjkuNTIxOUMxMDQuNTU3IDI5LjU3MjIgMTA0LjAzOCAyOS42MDE1IDEwMy4zNzMgMjkuNjAxNUMxMDEuNzUyIDI5LjYwMTUgMTAwLjU3NiAyOS4wMjc5IDk5Ljg0MzQgMjcuODc2NkM5OS40NjI0IDI3LjI2NTQgOTkuMTkwMyAyNi40MDI5IDk5LjAzNTQgMjUuMjg5M0M5OC4wNzY2IDI2LjU0NTMgOTYuNzAzNCAyNy42MzM4IDk0LjkwNzQgMjguNTU0OEM5My4xMTE0IDI5LjQ4MDEgOTEuMTM1MyAyOS45NDA2IDg4Ljk3NSAyOS45NDA2Qzg2LjM3OTQgMjkuOTQwNiA4NC4yNTY4IDI5LjE1MzUgODIuNjExNSAyNy41Nzk0QzgwLjk2NjEgMjYuMDA1MiA4MC4xNDE0IDI0LjAzMzMgODAuMTQxNCAyMS42Njc5QzgwLjE0MTQgMTkuMDc2NSA4MC45NTM2IDE3LjA2NjkgODIuNTczOCAxNS42MzkzQzg0LjE5NCAxNC4yMTE3IDg2LjMyMDggMTMuMzMyNSA4OC45NDk5IDEzLjAwMTdMOTYuNDM5NyAxMi4wNTU2Wk04Ni41MzQzIDI0Ljc2NkM4Ny41MjY1IDI1LjU0ODkgODguNzAyOSAyNS45MzgyIDkwLjA2MzUgMjUuOTM4MkM5MS43MTcyIDI1LjkzODIgOTMuMzIwNyAyNS41NTczIDk0Ljg3MzkgMjQuNzkxMUM5Ny40ODYzIDIzLjUyMjYgOTguNzkyNSAyMS40NDE5IDk4Ljc5MjUgMTguNTUzMVYxNC43Njg1Qzk4LjIxOSAxNS4xMzY5IDk3LjQ3NzkgMTUuNDQyNSA5Ni41NzM2IDE1LjY4NTNDOTUuNjY5MyAxNS45MjgyIDk0Ljc4MTggMTYuMTA0IDkzLjkxNTIgMTYuMjA4N0w5MS4wNzI1IDE2LjU3MjlDODkuMzY4NiAxNi43OTkgODguMDg3NSAxNy4xNTQ4IDg3LjIyOTIgMTcuNjQwNUM4NS43NzIzIDE4LjQ1NjggODUuMDQ4IDE5Ljc2MzEgODUuMDQ4IDIxLjU1MDdDODUuMDQzOCAyMi45MTU1IDg1LjU0MjEgMjMuOTgzMSA4Ni41MzQzIDI0Ljc2NloiIGZpbGw9IndoaXRlIi8+CjxwYXRoIGQ9Ik0xMTQuMTk1IDEuMDAzMDVWMTIuODkyOUwxMjYuMjYxIDEuMDAzMDVIMTMyLjI2OEwxMjEuNTA5IDExLjQxMDlMMTMyLjg3MSAyOC45OTQ0SDEyNi44NTlMMTE4LjA1NSAxNC43NTE3TDExNC4xOTEgMTguNDkwM1YyOC45OTg2SDEwOS42NjlWMS4wMDMwNUgxMTQuMTk1WiIgZmlsbD0id2hpdGUiLz4KPHBhdGggZD0iTTE1Mi4wNjIgMS43NzMzOEMxNTMuOTE3IDIuNzA2OTggMTU1LjMyOCAzLjkxMjcxIDE1Ni4yOTUgNS4zOTQ3NkMxNTcuMjI5IDYuODA1NjMgMTU3Ljg1MiA4LjQ1MDk1IDE1OC4xNjYgMTAuMzM0OUMxNTguNDQzIDExLjYyNDQgMTU4LjU4MSAxMy42OCAxNTguNTgxIDE2LjUwMTdIMTM4LjIyMkMxMzguMzEgMTkuMzQ4NiAxMzguOTc1IDIxLjYzNDQgMTQwLjIxOSAyMy4zNTUxQzE0MS40NjIgMjUuMDc1OCAxNDMuMzkyIDI1LjkzODIgMTQ2IDI1LjkzODJDMTQ4LjQzNyAyNS45MzgyIDE1MC4zODQgMjUuMTI2IDE1MS44MzYgMjMuNDk3NUMxNTIuNjQ5IDIyLjU1MTMgMTUzLjIyNiAyMS40NTg2IDE1My41NjUgMjAuMjE1MkgxNTguMTkyQzE1OC4wNyAyMS4yNDUxIDE1Ny42NjQgMjIuMzg4IDE1Ni45NzcgMjMuNjUyNEMxNTYuMjkxIDI0LjkxNjcgMTU1LjUyMSAyNS45NDY2IDE1NC42NzUgMjYuNzUwNEMxNTMuMjU2IDI4LjE0NDYgMTUxLjUwMSAyOS4wODY1IDE0OS40MDggMjkuNTcyMkMxNDguMjg2IDI5Ljg1MjcgMTQ3LjAxMyAyOS45OTA4IDE0NS41OTQgMjkuOTkwOEMxNDIuMTMyIDI5Ljk5MDggMTM5LjE5NyAyOC43MjIzIDEzNi43OSAyNi4xODk0QzEzNC4zODMgMjMuNjUyNCAxMzMuMTgxIDIwLjEwNjQgMTMzLjE4MSAxNS41Mzg4QzEzMy4xODEgMTEuMDQyNCAxMzQuMzkxIDcuMzkxNzUgMTM2LjgxOSA0LjU4Njc1QzEzOS4yNDMgMS43ODE3NSAxNDIuNDEyIDAuMzc5MjUzIDE0Ni4zMjcgMC4zNzkyNTNDMTQ4LjI5NCAwLjM3NTA2NyAxNTAuMjA4IDAuODQzOTYyIDE1Mi4wNjIgMS43NzMzOFpNMTUzLjc0NSAxMi43NjczQzE1My41NTcgMTAuNzI4NCAxNTMuMTEzIDkuMDk5ODcgMTUyLjQyMiA3Ljg4MTU4QzE1MS4xNDYgNS42MTY2NSAxNDkuMDEgNC40ODIwOSAxNDYuMDIxIDQuNDgyMDlDMTQzLjg3OCA0LjQ4MjA5IDE0Mi4wODIgNS4yNjA3OSAxNDAuNjMzIDYuODIyMzhDMTM5LjE4IDguMzgzOTcgMTM4LjQxNCAxMC4zNjQyIDEzOC4zMjYgMTIuNzY3M0gxNTMuNzQ1WiIgZmlsbD0id2hpdGUiLz4KPHBhdGggZD0iTTE2Ny4xODQgMS4wMDMwNVY0Ljk3NjFDMTY4LjUwNyAzLjMzOTE1IDE2OS45MSAyLjE2MjczIDE3MS4zOTIgMS40NDY4M0MxNzIuODc0IDAuNzMwOTI0IDE3NC41MTkgMC4zNzUwNjYgMTc2LjMzMiAwLjM3NTA2NkMxODAuMzA1IDAuMzc1MDY2IDE4Mi45ODkgMS43NjA4MiAxODQuMzgzIDQuNTMyMzNDMTg1LjE0OSA2LjA0Nzg2IDE4NS41MzQgOC4yMTY1IDE4NS41MzQgMTEuMDQyNFYyOC45OTg2SDE4MC43NDlWMTEuMzU2NEMxODAuNzQ5IDkuNjQ4MzEgMTgwLjQ5OCA4LjI3MDkzIDE3OS45OTEgNy4yMjg0N0MxNzkuMTU0IDUuNDg2ODYgMTc3LjYzOCA0LjYxNjA2IDE3NS40NDQgNC42MTYwNkMxNzQuMzMxIDQuNjE2MDYgMTczLjQxNCA0LjcyOTA5IDE3Mi42OTggNC45NTUxN0MxNzEuNDA4IDUuMzQwMzMgMTcwLjI3OCA2LjEwNjQ3IDE2OS4yOTggNy4yNTM1OUMxNjguNTE2IDguMTc4ODIgMTY4LjAwNSA5LjEyOTE3IDE2Ny43NyAxMC4xMTcyQzE2Ny41MzYgMTEuMTA1MiAxNjcuNDE5IDEyLjUwNzcgMTY3LjQxOSAxNC4zMzczVjI4Ljk5ODZIMTYyLjcxM1YxLjAwMzA1SDE2Ny4xODRaIiBmaWxsPSJ3aGl0ZSIvPgo8L3N2Zz4K" alt="Easywire logo">
      </div>
      <nav class="mt-5 flex-1 flex flex-col divide-y divide-cyan-800 overflow-y-auto" aria-label="Sidebar">
        <div class="px-2 space-y-1">
          <!-- Current: "bg-cyan-800 text-white", Default: "text-cyan-100 hover:text-white hover:bg-cyan-600" -->
          <a href="#" class="bg-cyan-800 text-white group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md" aria-current="page">
            <!-- Heroicon name: outline/home -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
            </svg>
            Home
          </a>

          <a href="#" class="text-cyan-100 hover:text-white hover:bg-cyan-600 group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md">
            <!-- Heroicon name: outline/clock -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            History
          </a>

          <a href="#" class="text-cyan-100 hover:text-white hover:bg-cyan-600 group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md">
            <!-- Heroicon name: outline/scale -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3" />
            </svg>
            Balances
          </a>

          <a href="#" class="text-cyan-100 hover:text-white hover:bg-cyan-600 group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md">
            <!-- Heroicon name: outline/credit-card -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
            </svg>
            Cards
          </a>

          <a href="#" class="text-cyan-100 hover:text-white hover:bg-cyan-600 group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md">
            <!-- Heroicon name: outline/user-group -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
            </svg>
            Recipients
          </a>

          <a href="#" class="text-cyan-100 hover:text-white hover:bg-cyan-600 group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md">
            <!-- Heroicon name: outline/document-report -->
            <svg class="mr-4 flex-shrink-0 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            Reports
          </a>
        </div>
        <div class="mt-6 pt-6">
          <div class="px-2 space-y-1">
            <a href="#" class="group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md text-cyan-100 hover:text-white hover:bg-cyan-600">
              <!-- Heroicon name: outline/cog -->
              <svg class="mr-4 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              Settings
            </a>

            <a href="#" class="group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md text-cyan-100 hover:text-white hover:bg-cyan-600">
              <!-- Heroicon name: outline/question-mark-circle -->
              <svg class="mr-4 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              Help
            </a>

            <a href="#" class="group flex items-center px-2 py-2 text-sm leading-6 font-medium rounded-md text-cyan-100 hover:text-white hover:bg-cyan-600">
              <!-- Heroicon name: outline/shield-check -->
              <svg class="mr-4 h-6 w-6 text-cyan-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
              </svg>
              Privacy
            </a>
          </div>
        </div>
      </nav>
    </div>
  </div>

  <div class="lg:pl-64 flex flex-col flex-1">
    <div class="relative z-10 flex-shrink-0 flex h-16 bg-white border-b border-gray-200 lg:border-none">
      <button type="button" class="px-4 border-r border-gray-200 text-gray-400 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-cyan-500 lg:hidden">
        <span class="sr-only">Open sidebar</span>
        <!-- Heroicon name: outline/menu-alt-1 -->
        <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" />
        </svg>
      </button>
      <!-- Search bar -->
      <div class="flex-1 px-4 flex justify-between sm:px-6 lg:max-w-6xl lg:mx-auto lg:px-8">
        <div class="flex-1 flex">
          <form class="w-full flex md:ml-0" action="#" method="GET">
            <label for="search-field" class="sr-only">Search</label>
            <div class="relative w-full text-gray-400 focus-within:text-gray-600">
              <div class="absolute inset-y-0 left-0 flex items-center pointer-events-none" aria-hidden="true">
                <!-- Heroicon name: solid/search -->
                <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                  <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
                </svg>
              </div>
              <input id="search-field" name="search-field" class="block w-full h-full pl-8 pr-3 py-2 border-transparent text-gray-900 placeholder-gray-500 focus:outline-none focus:ring-0 focus:border-transparent sm:text-sm" placeholder="Search transactions" type="search">
            </div>
          </form>
        </div>
        <div class="ml-4 flex items-center md:ml-6">
          <button type="button" class="bg-white p-1 rounded-full text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-500">
            <span class="sr-only">View notifications</span>
            <!-- Heroicon name: outline/bell -->
            <svg class="h-6 w-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9" />
            </svg>
          </button>

          <!-- Profile dropdown -->
          <div class="ml-3 relative">
            <div>
              <button type="button" class="max-w-xs bg-white rounded-full flex items-center text-sm focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-500 lg:p-2 lg:rounded-md lg:hover:bg-gray-50" id="user-menu-button" aria-expanded="false" aria-haspopup="true">
                <img class="h-8 w-8 rounded-full" src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80" alt="">
                <span class="hidden ml-3 text-gray-700 text-sm font-medium lg:block"><span class="sr-only">Open user menu for </span>{user.name}</span>
                <!-- Heroicon name: solid/chevron-down -->
                <svg class="hidden flex-shrink-0 ml-1 h-5 w-5 text-gray-400 lg:block" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                  <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
                </svg>
              </button>
            </div>

          </div>
        </div>
      </div>
    </div>
    <main class="flex-1 pb-8">
      <!-- Page header -->
      <div class="bg-white shadow">
        <div class="px-4 sm:px-6 lg:max-w-6xl lg:mx-auto lg:px-8">
          <div class="py-6 md:flex md:items-center md:justify-between lg:border-t lg:border-gray-200">
            <div class="flex-1 min-w-0">
              <!-- Profile -->
              <div class="flex items-center">
                <img class="hidden h-16 w-16 rounded-full sm:block" src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.6&w=256&h=256&q=80" alt="">
                <div>
                  <div class="flex items-center">
                    <img class="h-16 w-16 rounded-full sm:hidden" src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.6&w=256&h=256&q=80" alt="">
                    <h1 class="ml-3 text-2xl font-bold leading-7 text-gray-900 sm:leading-9 sm:truncate">
                      Good morning, {user.name}
                    </h1>
                  </div>
                  <dl class="mt-6 flex flex-col sm:ml-3 sm:mt-1 sm:flex-row sm:flex-wrap">
                    <dt class="sr-only">Company</dt>
                    <dd class="flex items-center text-sm text-gray-500 font-medium capitalize sm:mr-6">
                      <!-- Heroicon name: solid/office-building -->
                      <svg class="flex-shrink-0 mr-1.5 h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                        <path fill-rule="evenodd" d="M4 4a2 2 0 012-2h8a2 2 0 012 2v12a1 1 0 110 2h-3a1 1 0 01-1-1v-2a1 1 0 00-1-1H9a1 1 0 00-1 1v2a1 1 0 01-1 1H4a1 1 0 110-2V4zm3 1h2v2H7V5zm2 4H7v2h2V9zm2-4h2v2h-2V5zm2 4h-2v2h2V9z" clip-rule="evenodd" />
                      </svg>
                      Duke street studio
                    </dd>
                    <dt class="sr-only">Account status</dt>
                    <dd class="mt-3 flex items-center text-sm text-gray-500 font-medium sm:mr-6 sm:mt-0 capitalize">
                      <!-- Heroicon name: solid/check-circle -->
                      <svg class="flex-shrink-0 mr-1.5 h-5 w-5 text-green-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                      </svg>
                      Verified account
                    </dd>
                  </dl>
                </div>
              </div>
            </div>
            <div class="mt-6 flex space-x-3 md:mt-0 md:ml-4">
              <button type="button" class="inline-flex items-center px-4 py-2 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-cyan-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-cyan-500">
                Send money
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="mt-8">
        <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
          <h2 class="text-lg leading-6 font-medium text-gray-900">Overview</h2>
          <div class="mt-2 grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
            <!-- Card -->

            <div class="bg-white overflow-hidden shadow rounded-lg">
              <div class="p-5">
                <div class="flex items-center">
                  <div class="flex-shrink-0">
                    <!-- Heroicon name: outline/scale -->
                    <svg class="h-6 w-6 text-gray-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0M6 7l3 9M6 7l6-2m6 2l3-1m-3 1l-3 9a5.002 5.002 0 006.001 0M18 7l3 9m-3-9l-6-2m0-2v2m0 16V5m0 16H9m3 0h3" />
                    </svg>
                  </div>
                  <div class="ml-5 w-0 flex-1">
                    <dl>
                      <dt class="text-sm font-medium text-gray-500 truncate">
                        Account balance
                      </dt>
                      <dd>
                        <div class="text-lg font-medium text-gray-900">
                          $ {transactions.reduce((total, {amount}) => total + amount, 0)}
                        </div>
                      </dd>
                    </dl>
                  </div>
                </div>
              </div>
              <div class="bg-gray-50 px-5 py-3">
                <div class="text-sm">
                  <a href="#" class="font-medium text-cyan-700 hover:text-cyan-900">
                    View all
                  </a>
                </div>
              </div>
            </div>

            <!-- More items... -->
          </div>
        </div>

        <h2 class="max-w-6xl mx-auto mt-8 px-4 text-lg leading-6 font-medium text-gray-900 sm:px-6 lg:px-8">
          Recent activity
        </h2>

        <!-- Activity list (smallest breakpoint only) -->
        <div class="shadow sm:hidden">
          <ul role="list" class="mt-2 divide-y divide-gray-200 overflow-hidden shadow sm:hidden">
            {#each transactions as transaction (transaction.id)}
            <li>
              <a href="#" class="block px-4 py-4 bg-white hover:bg-gray-50">
                <span class="flex items-center space-x-4">
                  <span class="flex-1 flex space-x-2 truncate">
                    <!-- Heroicon name: solid/cash -->
                    <svg class="flex-shrink-0 h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                      <path fill-rule="evenodd" d="M4 4a2 2 0 00-2 2v4a2 2 0 002 2V6h10a2 2 0 00-2-2H4zm2 6a2 2 0 012-2h8a2 2 0 012 2v4a2 2 0 01-2 2H8a2 2 0 01-2-2v-4zm6 4a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd" />
                    </svg>
                    <span class="flex flex-col text-gray-500 text-sm truncate">
                      <span class="truncate">{transaction.note}</span>
                      <span><span class="text-gray-900 font-medium">${transaction.amount}</span> USD</span>
                      <time datetime="2020-07-11">{transaction.insertedAt}</time>
                    </span>
                  </span>
                  <!-- Heroicon name: solid/chevron-right -->
                  <svg class="flex-shrink-0 h-5 w-5 text-gray-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                    <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd" />
                  </svg>
                </span>
              </a>
            </li>
            {/each}

            <!-- More transactions... -->
          </ul>

          <nav class="bg-white px-4 py-3 flex items-center justify-between border-t border-gray-200" aria-label="Pagination">
            <div class="flex-1 flex justify-between">
              <a href="#" class="relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:text-gray-500">
                Previous
              </a>
              <a href="#" class="ml-3 relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:text-gray-500">
                Next
              </a>
            </div>
          </nav>
        </div>

        <!-- Activity table (small breakpoint and up) -->
        <div class="hidden sm:block">
          <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex flex-col mt-2">
              <div class="align-middle min-w-full overflow-x-auto shadow overflow-hidden sm:rounded-lg">
                <table class="min-w-full divide-y divide-gray-200">
                  <thead>
                    <tr>
                      <th class="px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Transaction
                      </th>
                      <th class="px-6 py-3 bg-gray-50 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Amount
                      </th>
                      <th class="hidden px-6 py-3 bg-gray-50 text-left text-xs font-medium text-gray-500 uppercase tracking-wider md:block">
                        Status
                      </th>
                      <th class="px-6 py-3 bg-gray-50 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                        Date
                      </th>
                    </tr>
                  </thead>
                  <tbody class="bg-white divide-y divide-gray-200">
                    {#each transactions as transaction (transaction.id)}
                    <tr class="bg-white">
                      <td class="max-w-0 w-full px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                        <div class="flex">
                          <a href="#" class="group inline-flex space-x-2 truncate text-sm">
                            <!-- Heroicon name: solid/cash -->
                            <svg class="flex-shrink-0 h-5 w-5 text-gray-400 group-hover:text-gray-500" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                              <path fill-rule="evenodd" d="M4 4a2 2 0 00-2 2v4a2 2 0 002 2V6h10a2 2 0 00-2-2H4zm2 6a2 2 0 012-2h8a2 2 0 012 2v4a2 2 0 01-2 2H8a2 2 0 01-2-2v-4zm6 4a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd" />
                            </svg>
                            <p class="text-gray-500 truncate group-hover:text-gray-900">
                              {transaction.note}
                            </p>
                          </a>
                        </div>
                      </td>
                      <td class="px-6 py-4 text-right whitespace-nowrap text-sm text-gray-500">
                        <span class="text-gray-900 font-medium">$ {transaction.amount}</span>
                        USD
                      </td>
                      <td class="hidden px-6 py-4 whitespace-nowrap text-sm text-gray-500 md:block">
                        <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800 capitalize">
                          success
                        </span>
                      </td>
                      <td class="px-6 py-4 text-right whitespace-nowrap text-sm text-gray-500">
                        <time datetime="2020-07-11">{transaction.insertedAt}</time>
                      </td>
                    </tr>
                    {/each}

                    <!-- More transactions... -->
                  </tbody>
                </table>
                <!-- Pagination -->
                <nav class="bg-white px-4 py-3 flex items-center justify-between border-t border-gray-200 sm:px-6" aria-label="Pagination">
                  <div class="hidden sm:block">
                    <p class="text-sm text-gray-700">
                      Showing
                      <span class="font-medium">{Math.min(1, transactions.length)}</span>
                      to
                      <span class="font-medium">{Math.min(10, transactions.length)}</span>
                      of
                      <span class="font-medium">{transactions.length}</span>
                      results
                    </p>
                  </div>
                  <div class="flex-1 flex justify-between sm:justify-end">
                    <a href="#" class="relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50">
                      Previous
                    </a>
                    <a href="#" class="ml-3 relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50">
                      Next
                    </a>
                  </div>
                </nav>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</div>
