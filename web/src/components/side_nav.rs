use leptos::*;

#[component]
pub fn Sidenav()->impl IntoView{
    view!{
        <div class="overflow-hidden fixed bottom-0 left-0 w-[200px] top-12 text-[13px] text-[#999999] bg-[#151515]">
            <div class="flex flex-col mb-3">
                <div class="h-[33px] flex items-center text-[#999999] uppercase pl-[12px] pr-[8px]">
                    <button class="material-symbols-outlined md-18 focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">draft</button>
                    <button class="material-symbols-outlined md-18 focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">search</button>
                    <button class="material-symbols-outlined md-18 focus:ring-1 focus:ring-violet-300 rounded-sm hover:bg-[#3c3c3c]/35 active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">refresh</button>
                </div>
                //folders
                <div id="folders">
                    <div class="flex items-center text-[#e5e5e5] text-[11px] uppercase px-[8px] h-[35px]">
                        <p class="pl-[12px]">EXPLORER</p>
                        <span class="material-symbols-outlined md-16 text-[#999999] w-[30px] ml-auto h-[25px] active:text-[#e5e5e5] cursor-pointer hover:text-[#e5e5e5] focus:text-[#e5e5e5]  p-[4px]">more_horiz</span>
                    </div>
                    <details class="flex flex-col">
                        <summary class="text-[#e5e5e5] mx-[1px] px-3 text-[11px] uppercase py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300">Downloads</summary>
                        <details>
                            <summary class="hover:bg-[#3c3c3c]/35 pl-6 pr-3 mx-[1px] cursor-pointer text-[#999999] active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">Music</summary>
                            <div class="mt-[1px] flex flex-col text-[13px] text-[#999999]">
                                <p class="hover:bg-[#3c3c3c]/35 pl-[32px] pr-[2px] mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">Don Toliver</p>
                                <p class="hover:bg-[#3c3c3c]/35 pl-[32px] pr-[2px] mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">New playlist</p>
                            </div>
                        </details>
                    </details>
                    <details class="flex flex-col">
                        <summary class="text-[#e5e5e5] mx-[1px] px-3 text-[11px] uppercase py-1 cursor-pointer hover:text-white active:text-white focus:text-white focus:ring-1 focus:ring-violet-300">Telegram-desktop</summary>
                        <details>
                            <summary class="hover:bg-[#3c3c3c]/35 pl-6 pr-3 mx-[1px] cursor-pointer text-[#999999] active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">Movies</summary>
                            <div class="mt-[1px] flex flex-col text-[13px] text-[#999999]">
                                <p class="hover:bg-[#3c3c3c]/35 pl-[32px] pr-[2px] mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">videos</p>
                                <p class="hover:bg-[#3c3c3c]/35 pl-[32px] pr-[2px] mx-[1px] cursor-pointer active:bg-[#37373D] hover:text-white active:text-white focus:text-white focus:bg-[#37373D] focus:ring-1 focus:ring-violet-300">Photos</p>
                            </div>
                        </details>
                    </details>
                </div>
                //search
                <div id="search"></div>
                
            </div>
        </div>
    }
} 