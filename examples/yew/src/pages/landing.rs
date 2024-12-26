use selectrs::yew::{Group, Option, Select};
use yew::prelude::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let selected_fruit = use_state(|| vec!["apple".to_string()]);
    let selected_veg = use_state(|| vec!["carrot".to_string()]);
    let selected_emojis = use_state(|| vec!["🎉".to_string()]);
    let onchange = |state: UseStateHandle<Vec<String>>| {
        Callback::from(move |value: Vec<String>| state.set(value))
    };

    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Select RS Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                // Default Headless Select
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Default Headless Select" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"<Select
    name="default-select"
    placeholder="Choose a fruit"
    onchange={Callback::from(move |value: Vec<String>| state.set(value))}
>
    <Group>
        <Option value="apple" label="🍎 Apple" />
        <Option value="banana" label="🍌 Banana" />
    </Group>
</Select>"# }
                    </pre>
                    <Select
                        name="default-select"
                        placeholder="Choose a fruit"
                        onchange={onchange(selected_fruit.clone())}
                    >
                        <Group>
                            <Option value="apple" label="🍎 Apple" />
                            <Option value="banana" label="🍌 Banana" />
                        </Group>
                    </Select>
                </div>
                // Default Disabled Select
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Headless Disabled Select" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"<Select
    name="disabled-select"
    placeholder="Disabled"
    disabled=true
>
    <Group>
        <Option value="apple" label="🍎 Apple" />
        <Option value="banana" label="🍌 Banana" />
    </Group>
</Select>"# }
                    </pre>
                    <Select name="disabled-select" placeholder="Disabled" disabled=true>
                        <Group>
                            <Option value="apple" label="🍎 Apple" />
                            <Option value="banana" label="🍌 Banana" />
                        </Group>
                    </Select>
                </div>
                // Disabled Options
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">{ "Disabled Options" }</h2>
                    <pre class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto">
                        { r#"<Select
    name="disabled-options"
    id="disabled-options"
    placeholder="Choose an option"
    class="w-full border border-gray-200 rounded-md p-2"
    select_class="w-full border border-gray-300 rounded-md p-2 focus:ring-2 focus:ring-green-500"
>
    <Group label="Fruits">
        <Option value="apple" label="Apple" disabled={true} />
        <Option value="banana" label="Banana" disabled={true} />
    </Group>
    <Group label="Vegetables">
        <Option value="carrot" label="Carrot" />
        <Option value="broccoli" label="Broccoli" />
    </Group>
</Select>"# }
                    </pre>
                    <Select
                        name="disabled-options"
                        id="disabled-options"
                        placeholder="Choose an option"
                        class="w-full border border-gray-200 rounded-md p-2"
                        select_class="w-full border border-gray-300 rounded-md p-2 focus:ring-2 focus:ring-green-500"
                    >
                        <Group label="Fruits">
                            <Option value="apple" label="Apple" disabled={true} />
                            <Option value="banana" label="Banana" disabled={true} />
                        </Group>
                        <Group label="Vegetables">
                            <Option value="carrot" label="Carrot" />
                            <Option value="broccoli" label="Broccoli" />
                        </Group>
                    </Select>
                </div>
                // Select with Custom Styles
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Custom Styled Select" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"<Select
    name="custom-styled-select"
    placeholder="Styled options"
    class="w-full border border-pink-300 rounded-md p-2 focus:ring-2 focus:ring-pink-500"
    select_class="w-full bg-pink-100 text-pink-900 p-2 shadow-lg rounded-lg"
>
    <Group>
        <Option value="rainbow" label="🌈 Rainbow" />
        <Option value="star" label="⭐ Star" />
    </Group>
</Select>"# }
                    </pre>
                    <Select
                        name="custom-styled-select"
                        placeholder="Styled options"
                        class="w-full border border-pink-300 rounded-md p-2 focus:ring-2 focus:ring-pink-500"
                        select_class="w-full bg-pink-100 text-pink-900 p-2 shadow-lg rounded-lg"
                    >
                        <Group>
                            <Option value="rainbow" label="🌈 Rainbow" />
                            <Option value="star" label="⭐ Star" />
                        </Group>
                    </Select>
                </div>
                // Grouped Select
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">{ "Grouped Select" }</h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"<Select
    name="emoji-select"
    placeholder="Pick an emoji"
    onchange={Callback::from(move |value: Vec<String>| state.set(value))}
    class="w-full border border-blue-300 rounded-md p-2 focus:ring-2 focus:ring-blue-500"
    select_class="w-full p-2 shadow-lg rounded-lg"
>
    <Group label="Faces" group=true>
        <Option value="😀" label="😀 Grinning Face" />
        <Option value="😂" label="😂 Laughing Face" />
    </Group>
    <Group label="Celebrations" group=true>
        <Option value="🎉" label="🎉 Party Popper" />
        <Option value="🎂" label="🎂 Birthday Cake" />
    </Group>
</Select>"# }
                    </pre>
                    <Select
                        name="emoji-select"
                        placeholder="Pick an emoji"
                        onchange={onchange(selected_emojis.clone())}
                        class="w-full border border-blue-300 rounded-md p-2 focus:ring-2 focus:ring-blue-500"
                        select_class="w-full p-2 shadow-lg rounded-lg"
                    >
                        <Group label="Faces" group=true>
                            <Option value="😀" label="😀 Grinning Face" />
                            <Option value="😂" label="😂 Laughing Face" />
                        </Group>
                        <Group label="Celebrations" group=true>
                            <Option value="🎉" label="🎉 Party Popper" />
                            <Option value="🎂" label="🎂 Birthday Cake" />
                        </Group>
                    </Select>
                </div>
                // 4. Multi-Select with Chips
                <div class="flex flex-col items-center bg-gray-50 p-6 rounded-lg shadow-lg">
                    <h2 class="text-xl font-semibold mb-4 text-gray-800">
                        { "Native Multi-Select with Chips" }
                    </h2>
                    <pre
                        class="font-mono text-xs text-gray-200 bg-gray-800 p-4 rounded-md w-full mb-4 overflow-x-auto"
                    >
                        { r#"<Select
    name="multi-select"
    placeholder="Select multiple"
    multiple=true
    onchange={Callback::from(move |value: Vec<String>| state.set(value))}
    class="w-full border border-green-300 rounded-md p-2 focus:ring-2 focus:ring-green-500"
    labels_class="flex flex-wrap gap-2 mt-2"
    label_class="bg-green-200 text-green-800 px-3 py-1 rounded-md"
    close_class="m-2 text-red-600 hover:text-red-800"
    select_class="w-full shadow-lg rounded-lg"
>
    <Group label="Vegetables">
        <Option value="🥕" label="🥕 Carrot" />
        <Option value="🥦" label="🥦 Broccoli" />
    </Group>
    <Group label="Fruits">
        <Option value="🍎" label="🍎 Apple" />
        <Option value="🍇" label="🍇 Grapes" />
    </Group>
</Select>"# }
                    </pre>
                    <Select
                        name="multi-select"
                        placeholder="Select multiple"
                        multiple=true
                        onchange={onchange(selected_veg.clone())}
                        class="w-full border border-green-300 rounded-md p-2 focus:ring-2 focus:ring-green-500"
                        labels_class="flex flex-wrap gap-2 mt-2"
                        label_class="bg-green-200 text-green-800 px-3 py-1 rounded-md"
                        close_class="m-2 text-red-600 hover:text-red-800"
                        select_class="w-full shadow-lg rounded-lg"
                    >
                        <Group label="Vegetables">
                            <Option value="🥕" label="🥕 Carrot" />
                            <Option value="🥦" label="🥦 Broccoli" />
                        </Group>
                        <Group label="Fruits">
                            <Option value="🍎" label="🍎 Apple" />
                            <Option value="🍇" label="🍇 Grapes" />
                        </Group>
                    </Select>
                </div>
            </div>
        </div>
    }
}
