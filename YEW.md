# Y Select RS Yew Usage

Adding Select RS to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the `select-rs` crate to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add selectrs --features=yew
   ```

1. Import the `Select`, `Option`, and `Group` components into your Yew component and start using them in your app.

## 🛠️ Usage

Incorporating Select RS into your application is easy. Follow these steps:

1. Import the `Select`, `Option`, and `Group` components into your Yew project:

   ```rust
   use yew::prelude::*;
   use selectrs::yew::{Select, Option, Group};
   ```

1. Use the `Select` component in your Yew application:

   ```rust
   use yew::prelude::*;
   use selectrs::yew::{Select, Option, Group};

   #[function_component(App)]
   pub fn app() -> Html {
       let onchange = Callback::from(|selected_values: Vec<String>| {
           log::info!("Selected: {:?}", selected_values);
       });

       html! {
           <Select
               class="custom-select"
               style="width: 250px"
               placeholder="Select an option..."
               onchange={onchange}
           >
               <Group>
                   <Option value="Option1" label="Option 1" />
                   <Option value="Option2" label="Option 2" />
                   <Option value="Option3" label="Option 3" />
               </Group>
           </Select>
       }
   }
   ```

## 🔧 Props

### Select Component

#### Main Props

| Property       | Type                       | Description                                                                   | Default |
| -------------- | -------------------------- | ----------------------------------------------------------------------------- | ------- |
| `name`         | `&'static str`             | The name attribute of the select component, important for form submission.    | `""`    |
| `id`           | `&'static str`             | The unique ID for the select element.                                         | `""`    |
| `placeholder`  | `&'static str`             | Placeholder text displayed when no option is selected.                        | `""`    |
| `multiple`     | `bool`                     | Whether multiple options can be selected.                                     | `false` |
| `required`     | `bool`                     | Marks the field as required for form submission.                              | `false` |
| `size`         | `u64`                      | Number of visible options in the dropdown (applies only for `multiple=true`). | `0`     |
| `form`         | `&'static str`             | Associates the select element with a specific form by its ID.                 | `""`    |
| `autocomplete` | `&'static str`             | Provides an autocomplete hint.                                                | `""`    |
| `autofocus`    | `bool`                     | Automatically focuses the select element on page load.                        | `false` |
| `children`     | `ChildrenWithProps<Group>` | Child `Group` components containing options to render within the select box.  | `""`    |

#### Styling Props

```sh
+--------------------------------------------------+
|                  [Select Container]              |  <-- `class` & `style`
|                                                  |
|   +------------------------------------------+   |
|   |             [Selected Labels]            |   |  <-- Rendered only for `multiple=true`
|   |                                          |   |
|   |   +----------------------------------+   |   |
|   |   |         [Label (Chip)]           |   |   |  <-- `label_class` & `label_style`
|   |   |    +--------------------------+  |   |   |
|   |   |    |    Close Button ("x")    |  |   |   |  <-- `close_class` & `close_style`
|   |   |    +--------------------------+  |   |   |
|   |   +----------------------------------+   |   |
|   +------------------------------------------+   |
|                                                  |
|   +------------------------------------------+   |
|   |              [Select Element]            |   |  <-- `<select>` element
|   |                                          |   |
|   |   +----------------------------------+   |   |
|   |   |      [Option (Placeholder)]      |   |   |  <-- Rendered if `placeholder` is set
|   |   +----------------------------------+   |   |
|   |                                          |   |
|   |   +----------------------------------+   |   |
|   |   |        [Options Group]           |   |   |  <-- Rendered dynamically via children
|   |   +----------------------------------+   |   |
|   +------------------------------------------+   |
|                                                  |
+--------------------------------------------------+
```

| Property       | Type           | Description                                    | Default |
| -------------- | -------------- | ---------------------------------------------- | ------- |
| `class`        | `&'static str` | CSS class for the outer select container.      | `""`    |
| `style`        | `&'static str` | Inline styles for the outer select container.  | `""`    |
| `labels_class` | `&'static str` | CSS class for the label container.             | `""`    |
| `labels_style` | `&'static str` | Inline styles for the label container.         | `""`    |
| `label_class`  | `&'static str` | CSS class for individual labels.               | `""`    |
| `label_style`  | `&'static str` | Inline styles for individual labels.           | `""`    |
| `close_class`  | `&'static str` | CSS class for the close button (multi-select). | `""`    |
| `close_style`  | `&'static str` | Inline styles for the close button.            | `""`    |
| `select_class` | `&'static str` | CSS class for the dropdown select box.         | `""`    |
| `select_style` | `&'static str` | Inline styles for the dropdown select box.     | `""`    |

#### Behavioral Props

| Property   | Type                    | Description                                         | Default |
| ---------- | ----------------------- | --------------------------------------------------- | ------- |
| `onchange` | `Callback<Vec<String>>` | Callback triggered when the selected values change. | No-op   |

### Group Component

#### Main Props

| Property   | Type                        | Description                                                       | Default |
| ---------- | --------------------------- | ----------------------------------------------------------------- | ------- |
| `label`    | `&'static str`              | Text label for the group, useful for describing a set of options. | `""`    |
| `group`    | `bool`                      | Indicates whether this is a group of options.                     | `false` |
| `selected` | `String`                    | The currently selected option within the group.                   | `""`    |
| `children` | `ChildrenWithProps<Option>` | Child `Option` components to display within this group.           | `""`    |

#### Styling Props

```sh
+--------------------------------------------------+
|               [OptGroup Container]               |  <-- Rendered if `group=true`
|              (`<optgroup>` element)              |
|   +------------------------------------------+   |
|   |         [Group Label/Text Header]        |   |  <-- `label` attribute (not styled)
|   +------------------------------------------+   |
|                                                  |
|   +------------------------------------------+   |
|   |              [Group Options]             |   |  <-- Rendered via child `Option` components
|   |   +----------------------------------+   |   |
|   |   |          [Option Label]          |   |   |
|   |   +----------------------------------+   |   |
|   +------------------------------------------+   |
|                                                  |
+--------------------------------------------------+
```

| Property | Type           | Description                            | Default |
| -------- | -------------- | -------------------------------------- | ------- |
| `class`  | `&'static str` | CSS class for the group container.     | `""`    |
| `style`  | `&'static str` | Inline styles for the group container. | `""`    |

#### Behavioral Props

| Property   | Type               | Description                                          | Default |
| ---------- | ------------------ | ---------------------------------------------------- | ------- |
| `onchange` | `Callback<String>` | Callback triggered when the selected option changes. | No-op   |

### Option Component

#### Main Props

| Property   | Type           | Description                                                        | Default |
| ---------- | -------------- | ------------------------------------------------------------------ | ------- |
| `value`    | `&'static str` | The underlying value associated with the option.                   | `""`    |
| `label`    | `Children`     | Content displayed for the option, such as text or custom elements. | None    |
| `selected` | `bool`         | Indicates if the option is currently selected.                     | `false` |
| `disabled` | `bool`         | Disables the option, making it unselectable by the user.           | `false` |

#### Styling Props

```sh
+--------------------------------------+
|         [Option Container]           |  <-- `<option>` element
|                                      |
|   +------------------------------+   |
|   |          [Label]             |   |  <-- Rendered dynamically via `label` prop
|   +------------------------------+   |
|                                      |
|      [Selected State Styling]        |  <-- Applies `selected_class` & `selected_style` if selected
+--------------------------------------+
```

| Property         | Type           | Description                                        | Default |
| ---------------- | -------------- | -------------------------------------------------- | ------- |
| `class`          | `&'static str` | CSS class for the option container.                | `""`    |
| `style`          | `&'static str` | Inline styles for the option container.            | `""`    |
| `selected_class` | `&'static str` | CSS class applied when the option is selected.     | `""`    |
| `selected_style` | `&'static str` | Inline styles applied when the option is selected. | `""`    |

#### Behavioral Props

| Property   | Type           | Description                                    | Default |
| ---------- | -------------- | ---------------------------------------------- | ------- |
| `on_click` | `Callback<()>` | Callback triggered when the option is clicked. | No-op   |

## 💡 Notes

- Use the `Group` component to organize related `Option` components within a `Select` component.
- The `onchange` callback is triggered with a list of selected values.
- If using the `multiple` prop, the `size` prop determines the number of visible options in the dropdown.
- Use `class` and `style` props to fully customize the appearance of the `Select` component, either with your own CSS or css libraries like Tailwind, Bootstrap, etc.
