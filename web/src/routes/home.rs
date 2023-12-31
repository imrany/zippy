use leptos::*;
use leptos_meta::*;
use wasm_bindgen::prelude::*;
use web_sys::{
 window,
};

#[path="../components/top_nav.rs"]
mod top_nav;
use top_nav::Topnav;

#[path="../components/side_nav.rs"]
mod side_nav;
use side_nav::Sidenav;

#[path="../components/bottom_nav.rs"]
mod bottom_nav;
use bottom_nav::Bottomnav;

#[path="../lib/functions.rs"]
mod functions;
use functions::{
    open_dialog,
};

// const ORANGE_ICON:&str =r#"
//     color: orange
// "#;

#[component]
pub fn Home() -> impl IntoView {
    let window=window().expect("Failed to get Window");
    let document=window.document().expect("Failed to get Document");
    let navigator=window.navigator();
    // window.alert_with_message(format!("Not online, {}",navigator.on_line()).as_str()).unwrap();

    if !navigator.on_line() {
        let closure: Closure<dyn FnMut()> = Closure::new(move|| {
            open_dialog("connection_dialog");
        });
        
        window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref()
        ,300).unwrap();
        closure.forget();
    }

    let document_ref_1=document.clone();
    let show_context_menu=move|_|{
        let context_list=document_ref_1.get_element_by_id("context_list").unwrap();
        context_list.class_list().toggle("block").unwrap();
    };

    let document_ref_2=document.clone();
    let open_more_menu=move|_|{
        let context_list=document_ref_2.get_element_by_id("context_more_share").unwrap();
        context_list.class_list().toggle("block").unwrap();
    };

    view! {
        <Title text="Welcome"/>
        <div class="min-h-[100vh] bg-[#1d1d1d]">
            <Topnav/>
            <Sidenav/>

            <div class="ml-[200px] mt-[48px] mb-[22px] text-[#999999]">
                // folder view
                <div id="folder_view">
                    // folder view nav
                    <div id="folder_view_nav">
                        <div class="flex w-full h-[35px] bg-[#151515]">
                            <div class="bg-[#1d1d1d] hover:bg-[#3c3c3c]/55 cursor-pointer pl-[10px] pr-[3px] min-w-[106px] h-[35px] flex items-center">
                                <span class="material-symbols-outlined md-16 w-[16px] mr-[5px]">folder_open</span>
                                <p class="text-[#E5E5E5] text-[13px]">Downloads</p>
                                <span class="material-symbols-outlined md-16 w-[22px] ml-[3px] p-[3px] hover:bg-gray-500 rounded-sm hover:text-white">close</span>
                            </div>

                            <div class="active:bg-[#1d1d1d] hover:bg-[#3c3c3c]/55 cursor-pointer pl-[10px] pr-[3px] min-w-[106px] h-[35px] flex items-center">
                                <span class="material-symbols-outlined md-16 w-[16px] mr-[5px]">folder</span>
                                <p class="text-[#E5E5E5] text-[13px]">telegram-desktop</p>
                                <span class="material-symbols-outlined md-16 w-[22px] p-[3px] ml-[3px] hover:bg-gray-500 rounded-sm hover:text-white">close</span>
                            </div>
                        </div>

                        <div class="w-full flex items-center h-[22px] text-[13px] text-[#999999] px-[16px]">
                            <p>src</p>
                            <span class="material-symbols-outlined md-16 w-[22px] p-[3px] hover:bg-gray-500 rounded-sm hover:text-white">chevron_right</span>
                            <p>telegram-desktop</p>
                        </div>
                    </div>

                    //folder view body 
                    <div class="w-full flex flex-wrap" id="folder_view_body">
                        <div class="flex grid max-sm:grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 w-full gap-4 px-[25px] py-[10px]">
                            <div class="flex">
                                <button on:click=show_context_menu.clone() class="flex flex-col items-center justify-center text-[12px] max-w-[150px] hover:text-white active:text-white focus:text-white">
                                    <img src="/assets/icons/file.png" alt="file" class="w-[75px] h-[75px]"/>
                                    <div>
                                        <p class="text-center">"y2mate.com - Gunna  COOLER THAN A BITCH feat Roddy Rich Official Audio.mp3"</p>
                                    </div>
                                </button>
                                <div id="context_list" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal ml-[80px] mt-[40px] z-5 py-[4px] dropdown-content absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">open_in_new</span>
                                        <p>Open</p>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">open_with</span>
                                        <p>Open with media player</p>
                                    </div>
                                    <div>
                                        <button on:click=open_more_menu.clone() class="btn_more pl-[12px] pr-[5px] py-[8px] w-full flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                            <span class="material-symbols-outlined md-16 pr-[6px]">share</span>
                                            <p>Share</p>
                                            <span class="material-symbols-outlined md-16 ml-auto">chevron_right</span>
                                        </button>
                                        <div id="context_more_share" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal ml-[191px] -mt-[10px] z-5 py-[4px] context-more-share absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                                            <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                                <span class="material-symbols-outlined md-16 pr-[6px]">bluetooth</span>
                                                <p>Bluetooth</p>
                                            </div>
                                            <div class="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                                <span class="material-symbols-outlined md-16 pr-[6px]">rss_feed</span>
                                                <p>"Send to"</p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">edit</span>
                                        <p>Rename</p>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">delete</span>
                                        <p>Delete</p>
                                    </div>
                                </div>
                            </div>

                            <div class="flex">
                                <button on:click=show_context_menu.clone() class="flex flex-col items-center justify-center text-[12px] max-w-[150px] hover:text-white active:text-white focus:text-white">
                                    <img src="/assets/icons/file.png" alt="file" class="w-[75px] h-[75px]"/>
                                    <div>
                                        <p class="text-center">"y2mate.com - Gunna  COOLER THAN A BITCH feat Roddy Rich Official Audio.mp3"</p>
                                    </div>
                                </button>
                                <div id="context_list" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal ml-[80px] mt-[40px] z-5 py-[4px] dropdown-content absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">open_in_new</span>
                                        <p>Open</p>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">open_with</span>
                                        <p>Open with media player</p>
                                    </div>
                                    <div>
                                        <button on:click=open_more_menu.clone() class="btn_more pl-[12px] pr-[5px] py-[8px] w-full flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                            <span class="material-symbols-outlined md-16 pr-[6px]">share</span>
                                            <p>Share</p>
                                            <span class="material-symbols-outlined md-16 ml-auto">chevron_right</span>
                                        </button>
                                        <div id="context_more_share" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal ml-[191px] -mt-[10px] z-5 py-[4px] context-more-share absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                                            <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                                <span class="material-symbols-outlined md-16 pr-[6px]">bluetooth</span>
                                                <p>Bluetooth</p>
                                            </div>
                                            <div class="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                                <span class="material-symbols-outlined md-16 pr-[6px]">rss_feed</span>
                                                <p>"Send to"</p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">edit</span>
                                        <p>Rename</p>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">delete</span>
                                        <p>Delete</p>
                                    </div>
                                </div>
                            </div>

                            <div class="flex">
                                <button on:click=show_context_menu.clone() class="flex flex-col items-center justify-center text-[12px] max-w-[150px] hover:text-white active:text-white focus:text-white">
                                    <img src="/assets/icons/file.png" alt="file" class="w-[75px] h-[75px]"/>
                                    <div>
                                        <p class="text-center">"y2mate.com - Gunna  COOLER THAN A BITCH feat Roddy Rich Official Audio.mp3"</p>
                                    </div>
                                </button>
                                <div id="context_list" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal ml-[80px] mt-[40px] z-5 py-[4px] dropdown-content absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">open_in_new</span>
                                        <p>Open</p>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">open_with</span>
                                        <p>Open with media player</p>
                                    </div>
                                    <div>
                                        <button on:click=open_more_menu.clone() class="btn_more pl-[12px] pr-[5px] py-[8px] w-full flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                            <span class="material-symbols-outlined md-16 pr-[6px]">share</span>
                                            <p>Share</p>
                                            <span class="material-symbols-outlined md-16 ml-auto">chevron_right</span>
                                        </button>
                                        <div id="context_more_share" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal ml-[191px] -mt-[10px] z-5 py-[4px] context-more-share absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                                            <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                                <span class="material-symbols-outlined md-16 pr-[6px]">bluetooth</span>
                                                <p>Bluetooth</p>
                                            </div>
                                            <div class="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                                <span class="material-symbols-outlined md-16 pr-[6px]">rss_feed</span>
                                                <p>"Send to"</p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">edit</span>
                                        <p>Rename</p>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">delete</span>
                                        <p>Delete</p>
                                    </div>
                                </div>
                            </div>

                            <div class="flex">
                                <button on:click=show_context_menu.clone() class="flex flex-col items-center justify-center text-[12px] max-w-[150px] hover:text-white active:text-white focus:text-white">
                                    <img src="/assets/icons/file.png" alt="file" class="w-[75px] h-[75px]"/>
                                    <div>
                                        <p class="text-center">"y2mate.com - Gunna  COOLER THAN A BITCH feat Roddy Rich Official Audio.mp3"</p>
                                    </div>
                                </button>
                                <div id="context_list" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal ml-[80px] mt-[40px] z-5 py-[4px] dropdown-content absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">open_in_new</span>
                                        <p>Open</p>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">open_with</span>
                                        <p>Open with media player</p>
                                    </div>
                                    <div>
                                        <button on:click=open_more_menu.clone() class="btn_more pl-[12px] pr-[5px] py-[8px] w-full flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                            <span class="material-symbols-outlined md-16 pr-[6px]">share</span>
                                            <p>Share</p>
                                            <span class="material-symbols-outlined md-16 ml-auto">chevron_right</span>
                                        </button>
                                        <div id="context_more_share" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal ml-[191px] -mt-[10px] z-5 py-[4px] context-more-share absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                                            <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                                <span class="material-symbols-outlined md-16 pr-[6px]">bluetooth</span>
                                                <p>Bluetooth</p>
                                            </div>
                                            <div class="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                                <span class="material-symbols-outlined md-16 pr-[6px]">rss_feed</span>
                                                <p>"Send to"</p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">edit</span>
                                        <p>Rename</p>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">delete</span>
                                        <p>Delete</p>
                                    </div>
                                </div>
                            </div>

                            <div class="flex">
                                <button on:click=show_context_menu.clone() class="flex flex-col items-center justify-center text-[12px] max-w-[150px] hover:text-white active:text-white focus:text-white">
                                    <img src="/assets/icons/file.png" alt="file" class="w-[75px] h-[75px]"/>
                                    <div>
                                        <p class="text-center">"y2mate.com - Gunna  COOLER THAN A BITCH feat Roddy Rich Official Audio.mp3"</p>
                                    </div>
                                </button>
                                <div id="context_list" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal ml-[80px] mt-[40px] z-5 py-[4px] dropdown-content absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">open_in_new</span>
                                        <p>Open</p>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">open_with</span>
                                        <p>Open with media player</p>
                                    </div>
                                    <div>
                                        <button on:click=open_more_menu.clone() class="btn_more pl-[12px] pr-[5px] py-[8px] w-full flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                            <span class="material-symbols-outlined md-16 pr-[6px]">share</span>
                                            <p>Share</p>
                                            <span class="material-symbols-outlined md-16 ml-auto">chevron_right</span>
                                        </button>
                                        <div id="context_more_share" style="box-shadow:0px 8px 16px 0px rgba(0,0,0,0.2);" class="font-normal ml-[191px] -mt-[10px] z-5 py-[4px] context-more-share absolute none bg-[#252525] min-w-[180px] rounded-[4px] text-white text-[13px]">
                                            <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                                <span class="material-symbols-outlined md-16 pr-[6px]">bluetooth</span>
                                                <p>Bluetooth</p>
                                            </div>
                                            <div class="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                                <span class="material-symbols-outlined md-16 pr-[6px]">rss_feed</span>
                                                <p>"Send to"</p>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">edit</span>
                                        <p>Rename</p>
                                    </div>
                                    <div class="px-[12px] py-[8px] flex items-center border-t-[1px] border-[#9999991A] cursor-pointer hover:bg-[#3c3c3c]/35 active:bg-[#3c3c3c]/35">
                                        <span class="material-symbols-outlined md-16 pr-[6px]">delete</span>
                                        <p>Delete</p>
                                    </div>
                                </div>
                            </div>
                            
                        </div>
                    </div>
                </div>

                // share tab
                <div id="share_tab"></div>
            </div>

            <Bottomnav/>
        </div>
    }
}