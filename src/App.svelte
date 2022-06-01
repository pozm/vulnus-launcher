
<script lang="ts">
	import Modal from './Components/Modal.svelte';
import { dialog,fs,invoke, path } from "@tauri-apps/api";

import IndexPage from "./lib/pages/IndexPage.svelte";
import { Data } from "./lib/store";
import { ShowPathModal,ShowSourceModal, VulnusPath, VersionsAvailable, LatestVersionsAvailable, ChosenVersion } from './lib/StoreData'
import {event} from '@tauri-apps/api'
import NotificationHandler from './Components/NotificationHandler.svelte';
import { onDestroy, onMount } from 'svelte';
import home_ico from './assets/svg/homeico.svg'
import settings_ico from './assets/svg/settingsico.svg'
import info_ico from './assets/svg/infoico.svg'
import add_ico from './assets/svg/plusico.svg'
import SettingsPage from './lib/pages/SettingsPage.svelte';
import ModPage from './lib/pages/ModPage.svelte';
import { fade, fly } from 'svelte/transition';
import InfoPage from './lib/pages/InfoPage.svelte';

	let updatePath = "";
	let updateSource = "";
	let PathActive = false;
	let awaitingData : ReturnType<typeof Data.Store.get.reload>
	onMount(async ()=>{

		await invoke("fetch_mods")

		awaitingData = Data.Store.get.reload();
		awaitingData.then(data=>{
			// updatePath = data["Vulnus.path"]
			console.log("got data: ",data)
			try{
				LatestVersionsAvailable.set(data.vulnus.version.latest)
				ChosenVersion.set(data.vulnus.version.current ?? data.vulnus.version.latest)
				VersionsAvailable.set(data.vulnus.version.versions)
			} catch {
				console.log("no versions")
			}
			//fg
			//f
			// if ((data["Vulnus.path"] ?? '') == '') {
			// 	ShowPathModal.set(true)
			// } else {
			// 	VulnusPath.set(data["Vulnus.path"]);
			// }
			event.emit("client://store-loaded")
		})
	})

	let PathIsInvalid = false;
	let PagesMap = [
		{c:IndexPage,s:home_ico,n:"Home"},
		{c:ModPage,s:add_ico,n:"Modding"},
		{c:InfoPage,s:info_ico,n:"Info"},
		{c:SettingsPage,s:settings_ico,n:"Settings"},
	]
	let ShowPage = 0;

	function GetVulnusPath() {
		console.log("GetVulnusPath");
		dialog.open({
			directory:true,
			defaultPath:updatePath,
			title:"Select Vulnus path",
			multiple:false
		}).then(v=>{
			updatePath = v as string;
		})
	}
	function getDefaultPath() {
		console.log("update")
		updatePath = Data.Store.get.data.vulnus.path
	}
	function SetVulnusPath() {
		invoke("set_path",{pathTo:updatePath}).then(()=>{
			Data.Store.get.reload();
			ShowPathModal.set(false);
			updatePath = "";
		})
	}
	function SetSource() {
		Data.Store.get.reload();
		ShowSourceModal.set(false);
		Data.Store.get.data.modding.source_list = updatePath
		Data.Store.get.write();
		// invoke("set_path",{pathTo:updatePath}).then(()=>{
		// })
	}
	$: {
		invoke<boolean>("dir_exist",{dir: updatePath}).then(d=>{
			PathIsInvalid=!d;
			console.log(PathIsInvalid)
		})
		if ($ShowPathModal == true && usableget) {
			usableget = false
			getDefaultPath()
		} else if ($ShowPathModal == false) {
			usableget = true;
		}
	}
	let usableget = true;
	let SidebarHovering = false;
	
	// onMount(()=>{
	// 	let bbb = 0;
	// 	event.emit("client://notification",{
	// 		title:"bruh",
	// 		data:`aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaapog${bbb++}`,
	// 		err:false
	// 	})
	// 	// interval = setInterval(()=>{
			
	// 	// },2e3)
	// })
	// let interval;
	
	// onDestroy(()=>{
	// 	// clearInterval(interval)
	// })
	
</script>
<div class="flex min-h-screen" >
	<NotificationHandler/>
	<Modal show={$ShowSourceModal} >
		<h1 class="text-gray-200 text-2xl" >Hi,</h1>
		<h3 class="text-gray-300 text-lg">To change the source of mods please enter the url to the source list</h3>
		<div class="relative">
			<label for="password" class="block text-sm font-medium text-gray-400" >Please enter the url to the source list</label>
			<input on:blur={()=>{
				PathActive=false;
			}} on:focus={(v)=>{
				PathActive=true
			}} bind:value={updateSource} id="path" name="path" class={`appearance-none w-full text-neutral-200 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border`} placeholder="https://....">
			<!-- <button on:click={GetVulnusPath} class="absolute right-2 bottom-2.5 w-6 h-6" >
				<svg xmlns="http://www.w3.org/2000/svg" class={`h-6 w-6 ${PathActive ? "text-pink-200" : "text-zinc-600"}`} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2" />
				</svg>
			</button> -->
		</div>
		<div class="flex justify-end" >
			<!-- {#if PathIsInvalid}
				<p class="mr-auto mt-2 text-sm text-red-400 select-none " >The path you have provided is is invalid </p>
				
			{/if} -->
			<div>
				<button class="py-2 shadow-sm px-8 transition-colors hover:bg-red-600 text-gray-100 bg-red-500 disabled:bg-red-600/50 mt-2 rounded-lg" on:click="{()=>{ShowPathModal.set(false)}}">Close</button>
				<button class="py-2 shadow-sm px-8 transition-colors hover:bg-green-600 text-gray-100 bg-emerald-500 disabled:bg-emerald-600/50 mt-2 rounded-lg" on:click="{SetSource}">Save</button>
			</div>
		</div>
	</Modal>
	<Modal show={$ShowPathModal} >
		<h1 class="text-gray-200 text-2xl" >Hi,</h1>
		<h3 class="text-gray-300 text-lg">To change the location of the <span class="text-pink-300" >Vulnus Launcher</span> please enter a directory</h3>
		<div class="relative">
			<label for="password" class="block text-sm font-medium text-gray-400" >Please enter the parent of the desired path, we will make a folder for vulnus.</label>
			<input on:blur={()=>{
				PathActive=false;
			}} on:focus={(v)=>{
				PathActive=true
			}} bind:value={updatePath} id="path" name="path" class={`appearance-none w-full text-neutral-200 placeholder-zinc-400 transition-colors bg-neutral-900 focus:outline-none focus:ring-pink-400 focus:border-pink-400 focus:ring-1 rounded-lg px-2 py-2 shadow-sm border ${PathIsInvalid ? "border-red-400 ring-red-400" : "border-zinc-600"}`} placeholder="Path of vulnus">
			<button on:click={GetVulnusPath} class="absolute right-2 bottom-2.5 w-6 h-6" >
				<svg xmlns="http://www.w3.org/2000/svg" class={`h-6 w-6 ${PathActive ? "text-pink-200" : "text-zinc-600"}`} fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
					<path stroke-linecap="round" stroke-linejoin="round" d="M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2" />
				</svg>
			</button>
		</div>
		<div class="flex justify-end" >
			{#if PathIsInvalid}
				<p class="mr-auto mt-2 text-sm text-red-400 select-none " >The path you have provided is is invalid </p>
				
			{/if}
			<div>
				<button class="py-2 shadow-sm px-8 transition-colors hover:bg-red-600 text-gray-100 bg-red-500 disabled:bg-red-600/50 mt-2 rounded-lg" on:click="{()=>{ShowPathModal.set(false)}}">Close</button>
				<button disabled={PathIsInvalid} class="py-2 shadow-sm px-8 transition-colors hover:bg-green-600 text-gray-100 bg-emerald-500 disabled:bg-emerald-600/50 mt-2 rounded-lg" on:click="{SetVulnusPath}">Save</button>
			</div>
		</div>
	</Modal>
	<div class="min-h-screen select-none w-14 bg-zinc-800 flex flex-col space-y-2 py-2 hover:w-32 transition-all duration-200 items-center hover:items-start px-1" on:mouseleave={()=>SidebarHovering=false} on:mouseenter={()=>SidebarHovering=true} >
		{#each PagesMap as page,i}
			{@const selected = ShowPage == i}
			<div class={`py-2 cursor-pointer w-full flex flex-row ${!SidebarHovering ? "justify-center" : "justify-start"} ${selected? "bg-zinc-900/50" : ""} hover:bg-zinc-900/90  rounded-lg px-2 lastBottom`} on:click={()=>ShowPage=i} >
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

<style scoped >
	.lastBottom:last-child {
		margin-top:auto !important;
	}
</style>