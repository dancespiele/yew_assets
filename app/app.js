import init, { run } from './pkg/yew_parcel.js';
async function main() {
   await init('/pkg/yew_parcel_bg.wasm');
   run();
}
main()