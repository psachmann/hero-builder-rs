use leptos::*;
use leptos_router::*;

#[component]
pub fn index() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <div class="container mx-auto bg-slate-100 rounded-lg shadow-md items-center">
                <div class="flex flex-row justify-center items-center p-4">
                    <A href="metadata" class="flex-1 text-blue-400 hover:underline">
                        {"Metadata"}
                    </A>
                    <A href="attributes" class="flex-1 text-blue-400 hover:underline">
                        {"Attributes"}
                    </A>
                    <A href="skills" class="flex-1 text-blue-400 hover:underline">
                        {"Skills"}
                    </A>
                    <A href="powers" class="flex-1 text-blue-400 hover:underline">
                        {"Powers"}
                    </A>
                    <A href="equipment" class="flex-1 text-blue-400 hover:underline">
                        {"Equipment"}
                    </A>
                </div>
            </div>
            <Outlet />
        </div>
    }
}
