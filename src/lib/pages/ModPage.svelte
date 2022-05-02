<script lang="ts">
import { event, invoke } from "@tauri-apps/api";

import { onDestroy, onMount } from "svelte";
import { each } from "svelte/internal";
import { get, writable } from "svelte/store";
import { fade } from "svelte/transition";
import type { IInstallProgress } from "../DataTypes";
import { getPercent } from "../SharedFunctions";
import { Data } from "../store";
import { formatRelative } from 'date-fns'
	let UnlistenToProgress : ReturnType<typeof event.listen>;
		let installingVersion = false;
	let installingText = "downloading"

	let installingTotal = 0;
	let installingProgress = 0;
	$: installingPercent = getPercent(installingTotal, installingProgress);
	onMount(()=>{

		UnlistenToProgress = event.listen<IInstallProgress>("server://install-progress",evData=>{
			console.log(evData)
			if (evData.payload.state == "Done") {
				installingText = "extracting"
			} else if (evData.payload.state == "Start") {
				installingText = "downloading"
				installingProgress = evData.payload.current
				installingTotal = evData.payload.total
				installingVersion = true
			} else {
				installingProgress = evData.payload.current
				installingTotal = evData.payload.total
			}
		})

	})

	onDestroy(()=>{
		UnlistenToProgress.then(e=>e());
	})

	let bepinexInstalled = isInstalled();

	function isInstalled() {
		return invoke("check_bepinex").then(r=>{
			return r
		},_=>false)
	}

	function installBepinex() {
		invoke("install_bepinex").then(()=>{
			installingVersion = false
		})
	}	

	function getRelativeDate(date) {
		return formatRelative(new Date(date), new Date())
	}

	function installMod(idx) {
		invoke("install_mod",{idx}).then(()=>Data.Store.get.reload(),console.error)
	}

	// let mods = writable([])
	$: mods = Data.Store.get.data.modding.mods
</script>


<h1 class="text-gray-200 text-5xl mb-4" >Modding</h1>
<div class="relative" >

	{#await bepinexInstalled then bInstalled}
	<!-- {@debug bInstalled} -->
		{#if bInstalled}
			<div class="grid w-full grid-cols-3" >
				{#each mods as mod,i}
					
					<div class="w-full h-52 bg-zinc-800 rounded-xl p-4 border border-solid border-zinc-600 shadow-lg text-neutral-400 flex flex-col" >
						<h3 class="text-xl" >{mod.name}</h3>
						<div class="w-full flex flex-col" >
							<ol>
								<li>
									Last updated: <span class="text-pink-300" > {getRelativeDate(mod.last_updated)} </span>
								</li>
								<li>
									Vulnus versions supported : <span class="text-pink-300" > {mod.available_versions.join(", ")} </span>
								</li>
							</ol>
						</div>
						<div class=" mt-auto w-full flex flex-row  justify-end" >
							<button disabled={mod?.installed ?? false}  on:click={installMod.bind(this,i)} class="ml-auto py-2 shadow-sm px-12 transition-colors hover:bg-emerald-600 text-gray-100 bg-emerald-500 disabled:bg-emerald-600/50 mt-2 rounded-lg">{mod?.installed ? "Installed" : "Install"}</button>
						</div>
					</div>
				{/each}
			</div>
		{:else}
			<button in:fade 
			on:click={installBepinex}
			class="py-2 w-full shadow-sm px-12 transition-colors hover:bg-emerald-600 text-gray-100 bg-emerald-500 disabled:bg-emerald-600/50 mt-2 rounded-lg"
			>Install</button
			>
			{#if installingVersion}
		
				<p for="downloading_vulnus" class="text-gray-400 select-none my-2" >
					{installingText}{ installingText == "downloading" ? `: ${installingPercent}` : ""}
				</p>
				<div class="w-full h-2 bg-zinc-800 rounded-lg relative overflow-hidden" >
					<div class="bg-pink-500 h-2" style:width={installingPercent} >
		
					</div>
				</div>
				<!-- <progress class="w-full rounded-lg" id="downloading_vulnus" value={installingProgress} max={installingTotal} /> -->
			{/if}
		{/if}
	{/await}


</div>