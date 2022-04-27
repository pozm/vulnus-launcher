<script lang="ts">
import { Command } from "@tauri-apps/api/shell";

import { launcherDir } from "../SharedFunctions";




	async function openLauncherPath() {
		let dir = await launcherDir();
		console.log(dir)
		new Command('ope',[dir]).spawn().then(ch=>{
			// the h
		});
	}

</script>

<div class="space-y-2" >

	<h1 class="text-gray-200 text-5xl mb-4" >Settings</h1>
	<div class="relative w-full">

		<div class="my-4 bg-neutral-900 p-2 border border-solid border-neutral-700 rounded-xl shadow-lg z-30" >
			
			<div class="flex flex-row w-full content-center items-center" >
				<p class="text-gray-300 text-xl mr-auto" >Vulnus Path: <span class="text-pink-300" >lol</span></p>
				<button  class="ml-auto py-2 shadow-sm px-12 transition-colors hover:bg-cyan-600 text-gray-100 bg-cyan-500 disabled:bg-cyan-600/50 mt-2 rounded-lg">Change</button>
			</div>
		<div class="flex flex-row w-full content-center items-center" >
			<p class="text-gray-300 text-xl mr-auto" >Open launcher folder</p>
			<button on:click={openLauncherPath} class="ml-auto py-2 shadow-sm px-12 transition-colors hover:bg-indigo-600 text-gray-100 bg-indigo-500 disabled:bg-indigo-600/50 mt-2 rounded-lg">Open</button>
		</div>
	</div>
</div>
</div>