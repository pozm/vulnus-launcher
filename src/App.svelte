
<script lang="ts">
	import Modal from './Components/Modal.svelte';
import { dialog,fs,invoke } from "@tauri-apps/api";

import IndexPage from "./lib/IndexPage.svelte";
import { Data } from "./lib/store";
import { ShowPathModal,ShowInstallModal, VulnusPath, VersionsAvailable, LatestVersionsAvailable } from './lib/StoreData'
import {event} from '@tauri-apps/api'
import NotificationHandler from './Components/NotificationHandler.svelte';
import { onDestroy, onMount } from 'svelte';

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
	let PagesMap = [IndexPage]
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
	<div class="min-h-screen w-14 bg-zinc-800 flex flex-col space-y-2 py-2 hover:w-32 transition-all items-center hover:items-start hover:pl-2" on:mouseenter={()=>SidebarHovering=true} >
		<div class="py-2 cursor-pointer flex flex-row" on:click={()=>ShowPage=0} >
			<svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7 text-gray-300" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<path stroke-linecap="round" stroke-linejoin="round" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
			</svg>
		</div>
		<div class="py-2 cursor-pointer" on:click={()=>ShowPage=1} >
			<svg xmlns="http://www.w3.org/2000/svg" class="h-7 w-7 text-gray-300" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
				<path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
				<path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
			  </svg>
		</div>
	</div>
	
	<div class="p-4 w-full" >
		<!-- <h1 class="text-gray-200 text-xl" >Vulnus Mod Assistant</h1> -->
		<svelte:component this={PagesMap[ShowPage]} />
	</div>
</div>