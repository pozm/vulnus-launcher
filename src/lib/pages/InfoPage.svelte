<script lang="ts">
import { Command } from "@tauri-apps/api/shell";




	function openGithub(user:string) {
		new Command('ope',[`https://github.com/${user}`]).spawn()
		// open(`https://github.com/${user}`)
	}

	let users = [
		{
			name: "Luna",
			desc:"Developer of the Vulnus Launcher",
			github : "pozm",
			color : "text-pink-300"
		},
		{
			name: "Hawk",
			desc:"Developer of Vulnus",
			github : "haawwkeye",
			color : "text-indigo-300"

		},
		{
			name: "Kermeet",
			desc:"Developer of Vulnus",
			github : "krmeet",
			color : "text-indigo-300"

		},
		{
			name: "TominoCZ",
			desc:"Developer of Sound Space",
			github : "TominoCZ",
			color : "text-sky-300"

		},
	]

</script>


<h1 class="text-gray-200 text-5xl mb-4" >Information</h1>
<p class="text-gray-400" >Hi, thanks for using the (unofficial) vulnus launcher!</p>
<hr class="my-2 border-zinc-700 border rounded-lg" />
<h3 class="text-gray-200 text-3xl mb-4" >Credits</h3>
<div class="block w-full" >
	<p class="text-gray-400" >Vulnus launcher wouldn't be possible without the people behind it</p>
	<ul class="text-gray-300 space-y-1 py-2 list-disc list-inside" >
		{#each users as user}
		<li><span class={`${user.color} hover:brightness-75 cursor-pointer`} on:click={openGithub.bind(this,user.github)} >{user.name}</span>: {user.desc}</li>

		{/each}
	</ul>
</div>