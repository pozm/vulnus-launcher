<script lang="ts">
import { event } from "@tauri-apps/api";
import type { Event } from "@tauri-apps/api/event";

import { onDestroy, onMount } from "svelte";
import type { IClientNotification } from "../lib/DataTypes";
import { fly, fade } from 'svelte/transition';
import { quintOut } from 'svelte/easing';


	let unregisterListener : Promise<event.UnlistenFn>;
	let notifications:Map<number,IClientNotification> = new Map()
	let nc = 0;
	onMount(()=>{
		unregisterListener = event.listen("client://notification",(notifi: Event<string>)=>{
			let parsedData:IClientNotification = JSON.parse(notifi.payload)
			console.log("got notification:",parsedData)
			let at = nc
			notifications.set(nc++,parsedData)
			notifications = notifications
			// console.log(`add ${at}`,notifications)
			setTimeout(()=>{
				notifications.delete(at)
				notifications = notifications
				if (notifications.size == 0) {
					nc=0;
				} 
				// console.log(`remove ${at}`,notifications)
			},4.3e3)
		})
	})
	onDestroy(()=>{
		unregisterListener.then(fn=>{
			fn(); 
		},e=>{
			console.log("unable to unlisten to notifications.")
		})
	})
	// let notifiArr: IClientNotification[]=[]
	$: revNoti = [...notifications].reverse();
</script>
<!-- {@debug notifications} -->
<div class="absolute h-full w-72 py-2 right-2" >
	<div class="w-full h-full flex flex-col" >
		{#each [...notifications] as [idx,notification]}
			<!-- {@debug notification} -->
			{#if notification?.data}
	
				<div in:fade out:fade class="shadow-xl bg-zinc-800 border border-solid border-neutral-600 rounded-xl p-2 mt-2 z-50 w-full" >
					<h2 class="text-gray-300 text-xl" >{notification.title}</h2>
					<p class="text-gray-400 text-sm w-full break-words" >{notification.data}</p>
				</div>
			{/if}
		{/each}
	</div>
</div>
<slot/>
