import { fs } from "@tauri-apps/api";
import { appDir } from "@tauri-apps/api/path";

const strToAB = (str) =>
    new Uint8Array(str.split("").map((c) => c.charCodeAt(0)));

const ABToStr = (ab) =>
    new Uint8Array(ab).reduce((p, c) => p + String.fromCharCode(c), "");

const DataUrl = async () => `${await appDir()}.data/pog/.pog`;

export namespace Data {

    type Pw = {[site:string]:string}

    export class Store {
        public data: Pw = {}
        private static instance: Store = new Store();
        private constructor () {
            read().then(dat=>this.data=dat)
        };
        static get get() {return this.instance};
        get entries() {
            return this.data;
        }
        async clear(){
            this.data = {};
            await write(this.data);
        }
        async add(key:string,val: string) {
            this.data[key] = val
            await write(this.data);
        }
        async write() {
            await write(this.data);
        }
        async reload() {
            await read().then(dat=>this.data=dat);
            return this.data
        }
        async del(site:string) {
            delete this.data[site];
            await write(this.data);
        }
    }

    function read() : Promise<Pw> {
        return new Promise(async (res, rj) => {
            fs.readBinaryFile(await DataUrl()).then(
                (data) => {
                    try {
                        res(JSON.parse(ABToStr(data)));
                        console.log("pog")
                    } catch {
                        res({});
                    }
                },
                async (fail) => {
                    await fs.createDir(`${await appDir()}.data/pog`, {
                        recursive: true,
                    });
                    fs.writeBinaryFile({
                        path: await DataUrl(),
                        contents: strToAB(JSON.stringify({})),
                    }).then(() => {
                        res({});
                    });
                }
            );
        });
    }
    function write(data: Pw) {
        return new Promise(async (res, rj) => {
            console.log("write",data)
            fs.writeBinaryFile({
                path: await DataUrl(),
                contents: strToAB(JSON.stringify(data)),
            }).then(() => {
                res([]);
            },fail=>{
                console.log("Write failure ",fail)
            });
        });
    }
}
