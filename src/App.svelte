
<script lang="ts">
	import Modal from './Components/Modal.svelte';
import { dialog,fs,invoke } from "@tauri-apps/api";

import IndexPage from "./lib/pages/IndexPage.svelte";
import { Data } from "./lib/store";
import { ShowPathModal,ShowInstallModal, VulnusPath, VersionsAvailable, LatestVersionsAvailable } from './lib/StoreData'
import {event} from '@tauri-apps/api'
import NotificationHandler from './Components/NotificationHandler.svelte';
import { onDestroy, onMount } from 'svelte';
import home_ico from './assets/svg/homeico.svg'
import settings_ico from './assets/svg/settingsico.svg'
import SettingsPage from './lib/pages/SettingsPage.svelte';
import { fade, fly } from 'svelte/transition';

	let updatePath = "";
	let PathActive = false;
	let awaitingData = Data.Store.get.reload();

	onMount(()=>{
		awaitingData.then(data=>{
			updatePath = data["Vulnus.path"]
			console.log("got data: ",data)
			try{
				LatestVersionsAvailable.set(data["Vulnus.versions.latest"])
				VersionsAvailable.set(JSON.parse(data["Vulnus.versions"]))
			} catch {
				console.log("no versions")
			}
			if ((data["Vulnus.path"] ?? '') == '') {
				ShowPathModal.set(true)
			} else {
				VulnusPath.set(data["Vulnus.path"]);
			}
			event.emit("client://store-loaded")
		})
	})

	let PathIsInvalid = false;
	let PagesMap = [{c:IndexPage,s:home_ico,n:"Home"},{c:SettingsPage,s:settings_ico,n:"Settings"}]
	let ShowPage = 0;

	let interval;


	let SidebarHovering = false;

	// onMount(()=>{
	// 	let bbb = 0;
	// 	interval = setInterval(()=>{
	// 		event.emit("client://notification",{title:"bruh",data:`aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaapog${bbb++}`})

	// 	},2e3)
	// })

	// onDestroy(()=>{
	// 	clearInterval(interval)
	// })

</script>
<div class="flex min-h-screen" >
	<NotificationHandler/>
	<div class="min-h-screen select-none w-14 bg-zinc-800 flex flex-col space-y-2 py-2 hover:w-32 transition-all duration-200 items-center hover:items-start hover:px-2" on:mouseleave={()=>SidebarHovering=false} on:mouseenter={()=>SidebarHovering=true} >
		{#each PagesMap as page,i}
			<div class={`py-2 cursor-pointer w-full flex flex-row ${!SidebarHovering ? "justify-center" : "justify-start"}`} on:click={()=>ShowPage=i} >
				<!-- {@debug page} -->
				<!-- <svg src /> -->
				<svelte:component class="h-7 w-7 text-gray-300" this={page.s} />
				{#if SidebarHovering}
				<p in:fly={{x:-20}} out:fly={{x:-20,duration:200}} class="text-neutral-400 flex items-center w-full justify-end" >{page.n}</p>
				
				{/if}
			</div>
		{/each}
	</div>
	
	<div class="p-4 w-full" >
		<!-- <h1 class="text-gray-200 text-xl" >Vulnus Mod Assistant</h1> -->
		<svelte:component this={PagesMap[ShowPage].c} />
	</div>
</div>