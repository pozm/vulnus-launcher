use maps_importer::install_map_remote;

#[tokio::main]
async fn main() {
	let instp = "C:\\Users\\Luna\\AppData\\Local\\vulnus-launcher\\v0.0.3";
	let res = install_map_remote(instp, "https://cdn.discordapp.com/attachments/966094481466216489/971209078045028372/xi_akasha.zip")
	.await.unwrap();

	println!("{}", res);
}