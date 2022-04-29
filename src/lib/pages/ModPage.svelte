<script lang="ts">
import { event, invoke } from "@tauri-apps/api";

import { onDestroy, onMount } from "svelte";
import { fade } from "svelte/transition";
import type { IInstallProgress } from "../DataTypes";
import { getPercent } from "../SharedFunctions";

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

	function installBepinex() {
		invoke("install_bepinex").then(()=>{
			installingVersion = false
		})
	}

</script>


<h1 class="text-gray-200 text-5xl mb-4" >Modding</h1>
<div class="relative" >

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

</div>