
import type Tab from "types/tab";


export const TABS_KEY = "tabs";


export const getTabs = () =>  JSON.parse(localStorage.getItem(TABS_KEY) || "[]") as unknown as Tab[];

export const saveTabs = (tabs: Tab[]) => localStorage.setItem(TABS_KEY, JSON.stringify(tabs || []));


