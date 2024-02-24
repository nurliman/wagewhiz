import { writable } from "svelte/store";
import { persist, createLocalStorage } from "svelte-persistent-store";

export const isLoggedIn = persist<boolean>(writable(false), createLocalStorage(), "isLoggedIn");
