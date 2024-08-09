import { writable } from "svelte/store";

interface OpenTab {
    name: string;
    content: string;
}

export const activeDoc = writable<string | null>(null);

export const openTabs = writable<OpenTab[]>([]);
