use std::rc::Rc;
use yew::prelude::*;

/// Properties for configuring the `Select` component.
///
/// The `Select` component creates a customizable dropdown list that allows you to choose
/// a single or multiple options. It can be styled with custom classes and inline styles,
/// and supports additional behaviors like multiple selections, disabled options, and
/// change events for updating the selected value.
///
/// It works in combination with `Group` and `Option` components to provide a rich UI for
/// selecting options from a list.
/// Refer to the [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select#attributes) for more info.
#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps {
    /// The name of the select component.
    ///
    /// This represents the name attribute used in the underlying HTML `select` element.
    /// It is important when the component is part of a form, as it defines the field name.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub name: &'static str,

    /// The id of the select component.
    ///
    /// This represents the id attribute used in the underlying HTML `select` element.
    /// It helps in uniquely identifying the component within the DOM.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub id: &'static str,

    /// The placeholder text for the select component.
    ///
    /// This text is displayed when no option is selected and the `select` element is empty.
    /// It provides a hint to the user on what to select. It is not visible after an option is chosen.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub placeholder: &'static str,

    /// Whether the select component allows multiple selections.
    ///
    /// If set to `true`, the user can select more than one option. If set to `false`, only one option can be selected at a time.
    /// Defaults to `false` if not provided.
    #[prop_or_default]
    pub multiple: bool,

    /// Whether the select component is disabled.
    ///
    /// If set to `true`, the select component will be unresponsive and users will not be able to interact with it.
    /// Defaults to `false` if not provided.
    #[prop_or_default]
    pub disabled: bool,

    /// Whether the select option is required.
    ///
    /// This Boolean attribute indicates that a value must be selected from the dropdown.
    /// If not selected, form submission will be blocked. Defaults to `false` if not provided.
    #[prop_or_default]
    pub required: bool,

    /// The visible size of the select dropdown.
    ///
    /// If the `multiple` attribute is specified, this defines the number of visible rows in the list.
    /// This is helpful when displaying a scrollable list of options.
    /// Defaults to `0`, which means the default layout will be used.
    #[prop_or(0)]
    pub size: u64,

    /// The form to associate the select element with.
    ///
    /// This attribute allows you to associate the select element with a form elsewhere in the document.
    /// The value must be the `id` of a form element in the same document. If not provided, the `select`
    /// element will be associated with its nearest ancestor form. Defaults to an empty string if not provided.
    #[prop_or_default]
    pub form: &'static str,

    /// The autocomplete hint for the select element.
    ///
    /// This string provides a hint to the user agent's autocomplete feature, helping it to
    /// pre-fill values based on the user's past selections. The value should match one of the
    /// valid autocomplete values for the `<select>` element. Defaults to an empty string if not provided.
    #[prop_or_default]
    pub autocomplete: &'static str,

    /// Automatically focuses the select element when the page loads.
    ///
    /// This Boolean attribute lets you specify that the select element should automatically
    /// gain input focus when the page loads. Only one form element in a document can have this attribute.
    /// Defaults to `false` if not provided.
    #[prop_or_default]
    pub autofocus: bool,

    /// Callback triggered when the selected values change.
    ///
    /// This callback is executed whenever the user selects or deselects an option in the select box.
    /// It receives a vector of strings representing the selected options. This is useful for updating
    /// the selected values in the application state. Defaults to a no-op if not provided.
    #[prop_or_default]
    pub onchange: Callback<Vec<String>>,

    /// Child components for the select component.
    ///
    /// This property allows you to pass one or more `Group` components as children of the `Select` component.
    /// The `Group` components contain the `Option` components, which represent the individual selectable options.
    /// Defaults to an empty list of children if not provided.
    #[prop_or_default]
    pub children: ChildrenWithProps<Group>,

    /// Custom CSS class for the select container.
    ///
    /// This property allows for custom styling of the select container by specifying one or more CSS classes.
    /// It is applied to the outer wrapper of the `select` element. Defaults to an empty string if not provided.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline styles for the select container.
    ///
    /// This property allows for custom inline styles to be applied directly to the select container.
    /// It provides more granular control over the styling of the component, without the need for external CSS.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub style: &'static str,

    /// Custom CSS class for the label container.
    ///
    /// This property allows for custom styling of the labels in the `Select` component. It applies to the wrapper
    /// around the labels (for multi-selects or grouped selections). Defaults to an empty string if not provided.
    #[prop_or_default]
    pub labels_class: &'static str,

    /// Inline styles for the label container.
    ///
    /// This property allows for custom inline styles to be applied directly to the label container. This is useful
    /// for modifying the appearance of the labels within the select dropdown.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub labels_style: &'static str,

    /// Custom CSS class for the individual labels.
    ///
    /// This property allows for custom styling of the labels within the dropdown options.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub label_class: &'static str,

    /// Inline styles for the individual labels.
    ///
    /// This property allows for custom inline styles to be applied directly to the individual labels.
    /// It can be used to adjust the style of each label element within the `select` component.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub label_style: &'static str,

    /// Custom CSS class for the close button (for multi-select).
    ///
    /// This property allows for custom styling of the close button that appears next to selected values in a multi-select dropdown.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub close_class: &'static str,

    /// Inline styles for the close button (for multi-select).
    ///
    /// This property allows for custom inline styles to be applied directly to the close button.
    /// This can be used to change the appearance of the button that removes selected options in a multi-select.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub close_style: &'static str,

    /// Custom CSS class for the select dropdown.
    ///
    /// This property allows for custom styling of the select dropdown box itself. This class is applied to the
    /// `select` element in the rendered HTML. Defaults to an empty string if not provided.
    #[prop_or_default]
    pub select_class: &'static str,

    /// Inline styles for the select dropdown.
    ///
    /// This property allows for custom inline styles to be applied directly to the select dropdown.
    /// It gives more granular control over the dropdown's appearance, such as height, width, or border color.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub select_style: &'static str,
}

/// Select Component
///
/// A Yew component for creating a customizable select dropdown with support for single or multiple selections.
/// The `Select` component can handle options, dynamically manage selection, and customize its appearance and behavior.
///
/// # Properties
/// The component uses the `SelectProps` struct for its properties. Key properties include:
///
/// - **name**: The name of the select element (`&'static str`). Default: `""`.
/// - **id**: The ID of the select element (`&'static str`). Default: `""`.
/// - **placeholder**: Placeholder text for the select input when no options are selected (`&'static str`). Default: `""`.
/// - **multiple**: Whether the select allows multiple selections (`bool`). Default: `false`.
/// - **disabled**: Whether the select element is disabled (`bool`). Default: `false`.
/// - **onchange**: Callback triggered when the selected values change (`Callback<Vec<String>>`). Default: no-op.
/// - **children**: A collection of `Option` components as children (`ChildrenWithProps<Option>`). Default: empty.
/// - **class**: Custom CSS class for the select container (`&'static str`). Default: `""`.
/// - **style**: Inline styles for the select container (`&'static str`). Default: `""`.
/// - **labels_class**: Custom class for the selected options' labels (`&'static str`). Default: `""`.
/// - **labels_style**: Inline styles for the selected options' labels (`&'static str`). Default: `""`.
/// - **label_class**: Custom class for each label when an option is selected (`&'static str`). Default: `""`.
/// - **label_style**: Inline styles for each label when an option is selected (`&'static str`). Default: `""`.
/// - **close_class**: Custom class for the close button (`&'static str`). Default: `""`.
/// - **close_style**: Inline styles for the close button (`&'static str`). Default: `""`.
/// - **select_class**: Custom CSS class for the select element itself (`&'static str`). Default: `""`.
/// - **select_style**: Inline styles for the select element (`&'static str`). Default: `""`.
/// - **size**: The number of visible options in a scrolling select (`usize`). Default: `0`.
/// - **required**: Whether the select field is required (`bool`). Default: `false`.
/// - **form**: The ID of the form that the select is associated with (`&'static str`). Default: `""`.
/// - **autocomplete**: Hint for the browser's autocomplete feature (`&'static str`). Default: `""`.
/// - **autofocus**: Whether the select should gain focus when the page loads (`bool`). Default: `false`.
///
/// # Features
/// - Supports both single and multiple selection modes.
/// - Customizable via CSS classes and inline styles.
/// - Optionally displays a placeholder and manages selected items with chips (for multiple selections).
/// - Trigger an `onchange` callback whenever the selection changes.
///
/// # Examples
///
/// ## Basic Usage
/// ```rust
/// use yew::prelude::*;
/// use selectrs::yew::{Select, Option, Group};
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let onchange = Callback::from(|selected_values: Vec<String>| {
///         // Handle the selected values
///         log::info!("Selected: {:?}", selected_values);
///     });
///
///     html! {
///         <Select onchange={onchange}>
///             <Group>
///                 <Option value="Option1" label="Option 1" />
///                 <Option value="Option2" label="Option 2" />
///                 <Option value="Option3" label="Option 3" />
///             </Group>
///         </Select>
///     }
/// }
/// ```
///
/// ## Multiple Selection
/// ```rust
/// use yew::prelude::*;
/// use selectrs::yew::{Select, Option, Group};
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     let onchange = Callback::from(|selected_values: Vec<String>| {
///         // Handle the selected values
///         log::info!("Selected: {:?}", selected_values);
///     });
///
///     html! {
///         <Select multiple=true onchange={onchange}>
///             <Group>
///                 <Option value="Option1" label="Option 1" />
///                 <Option value="Option2" label="Option 2" />
///                 <Option value="Option3" label="Option 3" />
///             </Group>
///         </Select>
///     }
/// }
/// ```
///
/// ## Custom Styling and Placeholder
/// ```rust
/// use yew::prelude::*;
/// use selectrs::yew::{Select, Option, Group};
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Select
///             placeholder="Select an option..."
///             class="custom-select"
///             style="width: 200px"
///             size={5}
///         >
///             <Group>
///                 <Option value="Option1" label="Option 1" />
///                 <Option value="Option2" label="Option 2" />
///             </Group>
///         </Select>
///     }
/// }
/// ```
///
/// ## With a Required Field
/// ```rust
/// use yew::prelude::*;
/// use selectrs::yew::{Select, Option, Group};
///
/// #[function_component(App)]
/// pub fn app() -> Html {
///     html! {
///         <Select
///             required=true
///             onchange={Callback::from(|selected_values: Vec<String>| {
///                 log::info!("Selected: {:?}", selected_values);
///             })}
///         >
///             <Group>
///                 <Option value="Option1" label="Option 1" />
///                 <Option value="Option2" label="Option 2" />
///             </Group>
///         </Select>
///     }
/// }
/// ```
///
/// # Behavior
/// - The `Select` component handles single and multiple selections dynamically.
/// - The selected values are updated using the `onchange` callback whenever the user interacts with the select options.
/// - When multiple selection mode is enabled, selected values are displayed as chips with a close button to remove individual selections.
/// - A placeholder option is displayed when no value is selected and the select is not disabled.
///
/// # Notes
/// - The `children` property must contain `Option` components to populate the select dropdown.
/// - If the `multiple` property is `true`, multiple options can be selected at once. If `false`, only one option can be selected.
/// - Custom styling can be applied to the select container, options, and labels via CSS classes or inline styles.
#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let SelectProps {
        name,
        id,
        placeholder,
        multiple,
        disabled,
        onchange,
        children,
        class,
        style,
        labels_class,
        labels_style,
        label_class,
        label_style,
        close_class,
        close_style,
        select_class,
        select_style,
        size,
        required,
        form,
        autocomplete,
        autofocus,
    } = props.clone();

    let selected_values = use_state(Vec::<String>::new);
    let selected = (*selected_values).clone();

    let handle_group_change = {
        let selected_values = selected_values.clone();
        let on_change = onchange.clone();
        Callback::from(move |value: String| {
            let mut current_values = (*selected_values).clone();
            if multiple {
                if current_values.contains(&value) {
                    current_values.retain(|v| v != &value);
                } else {
                    current_values.push(value);
                }
            } else {
                current_values = vec![value];
            }
            selected_values.set(current_values.clone());
            on_change.emit(current_values);
        })
    };

    let remove_chip = {
        let selected_values = selected_values.clone();
        let on_change = onchange.clone();
        Callback::from(move |value: String| {
            let mut current_values = (*selected_values).clone();
            current_values.retain(|v| v != &value);
            selected_values.set(current_values.clone());
            on_change.emit(current_values);
        })
    };

    html! {
        <div class={class} style={style}>
            { if multiple {
                html! {
                    <div class={labels_class} style={labels_style}>
                        { for selected.clone().into_iter().map(|value| html! {
                            <div class={label_class} style={label_style}>
                                { value.clone() }
                                <button class={close_class} style={close_style} onclick={remove_chip.clone().reform(move |_| value.clone())}>
                                    { "x" }
                                </button>
                            </div>
                        }) }
                    </div>
                }
            } else {
                html! {}
            } }
            <select
                id={id}
                name={name}
                multiple={multiple}
                class={select_class}
                style={select_style}
                disabled={disabled}
                size={size.to_string()}
                required={required}
                form={form}
                autocomplete={autocomplete}
                autofocus={autofocus}
            >
                { if (!placeholder.is_empty() && selected.is_empty()) || disabled {
                    html! { <option disabled=true selected=true>{ placeholder }</option> }
                } else {
                    html! {}
                } }
                if !disabled {
                    { for children.iter().map(|mut child| {
                    let props = Rc::make_mut(&mut child.props);

                    props.selected = selected.first().cloned().unwrap_or_default();
                    let handle_group_change = handle_group_change.clone();
                    props.onchange = Callback::from(move |value| handle_group_change.emit(value));

                    child
                }) }
                }
            </select>
        </div>
    }
}

/// Properties for configuring the `Group` component.
///
/// The `Group` component allows you to group together `Option` elements.
/// It provides customization for labels, selection behavior, and event handling. The group allows a common state and
/// behavior across the contained options. This component supports customization of styles and classes, as well as
/// interaction handling through the `onchange` callback.
#[derive(Properties, PartialEq, Clone)]
pub struct GroupProps {
    /// The label for the group.
    ///
    /// This is the text that labels the entire group of options. This can be used to describe
    /// the set of options the user is about to choose from, making it useful for accessibility.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub label: &'static str,

    /// Indicates whether this is a group options.
    ///
    /// If `group` is set to `true`, the options in this group will be considered as part of the
    /// `label` options group, where only one option can be selected at a time. If set to `false`, the
    /// group is disabled. Defaults to `false` if not provided.
    #[prop_or_default]
    pub group: bool,

    /// The currently selected option in the group.
    ///
    /// This represents the selected value within the group. It should be bound to a state to
    /// reflect changes as the user selects different options. Defaults to an empty string if not provided.
    #[prop_or_default]
    pub selected: String,

    /// Callback for when the selected option changes.
    ///
    /// This callback is triggered whenever the user selects a different option within the group.
    /// The callback receives a string representing the newly selected option's value. Defaults to a no-op.
    #[prop_or_default]
    pub onchange: Callback<String>,

    /// Child components of type `Option` for the group.
    ///
    /// This property allows you to pass one or more `Option` components as children of the `Group` component.
    /// These `Option` components represent the individual selectable options within the group.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub children: ChildrenWithProps<Option>,

    /// Custom CSS class for the group.
    ///
    /// This property allows for custom styling of the group container by specifying one or more CSS classes.
    /// It is applied to the outer wrapper of the group, such as for styling the container element.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline styles for the group.
    ///
    /// This property allows for custom inline styles to be applied directly to the group container.
    /// It provides more granular control over the styling of the group and its elements without the need for external CSS.
    /// Defaults to an empty string if not provided.
    #[prop_or_default]
    pub style: &'static str,
}

#[function_component(Group)]
pub fn group(props: &GroupProps) -> Html {
    let GroupProps {
        label,
        group,
        selected,
        onchange,
        children,
        class,
        style,
    } = props.clone();

    if group {
        html! {
            <optgroup label={label} class={class} style={style}>
                { for children.iter().map(|mut child| {
                    let props = Rc::make_mut(&mut child.props);
                    let is_selected = props.value == selected;
                    let onchange = onchange.clone();
                    let value = props.value;

                    props.selected = is_selected;
                    props.on_click = Callback::from(move |_| {
                        onchange.emit(value.to_string());
                    });

                    child
                }) }
            </optgroup>
        }
    } else {
        html! {
            { for children.iter().map(|mut child| {
                    let props = Rc::make_mut(&mut child.props);
                    let is_selected = props.value == selected;
                    let onchange = onchange.clone();
                    let value = props.value;

                    props.selected = is_selected;
                    props.on_click = Callback::from(move |_| {
                        onchange.emit(value.to_string());
                    });

                    child
                }) }
        }
    }
}

/// Properties for configuring the `Option` component.
///
/// The `Option` component represents an individual selectable option within a group of options, such as a
/// string, a radio button, checkbox, or a custom select element. It allows for customization of the option's value,
/// label, selection state, and appearance. The component also supports event handling for user interactions
/// (e.g., click events).
#[derive(Properties, PartialEq, Clone)]
pub struct OptionProps {
    /// The value of the option.
    ///
    /// This is the underlying value associated with the option. It is typically used when the user selects
    /// this option, and is submitted or processed based on the selected state of the option. Defaults to an
    /// empty string if not provided.
    #[prop_or_default]
    pub value: &'static str,

    /// The label displayed for the option.
    ///
    /// This property defines the content that is shown to the user as the label for the option. It can be
    /// any valid child element, such as text, icons, or other components. Defaults to no label (empty).
    #[prop_or_default]
    pub label: Children,

    /// Whether the option is selected.
    ///
    /// This property indicates if the option is currently selected. If set to `true`, the option is visually
    /// marked as selected, and it may trigger related behavior such as updating state or submitting a form.
    /// Defaults to `false` if not provided.
    #[prop_or_default]
    pub selected: bool,

    /// Whether the option is disabled.
    ///
    /// If set to `true`, the option is considered disabled, meaning it cannot be interacted with by the user.
    /// Disabled options may be visually different (e.g., grayed out). Defaults to `false` if not provided.
    #[prop_or_default]
    pub disabled: bool,

    /// Callback for when the option is clicked.
    ///
    /// This callback is invoked when the user clicks on the option. It can be used to trigger actions such as
    /// updating the selected state or performing any other interaction. Defaults to a no-op (no action).
    #[prop_or_default]
    pub on_click: Callback<()>,

    /// Custom CSS class for the option.
    ///
    /// This property allows you to specify a custom CSS class for the option. This class is applied to the
    /// individual option container, enabling you to style it differently from other options. Defaults to an empty
    /// string if not provided.
    #[prop_or_default]
    pub class: &'static str,

    /// Inline styles for the option.
    ///
    /// This property enables you to apply inline styles directly to the option element. It allows for precise
    /// customization of the option's appearance without needing an external stylesheet. Defaults to an empty string
    /// if not provided.
    #[prop_or_default]
    pub style: &'static str,

    /// Custom class for a selected option.
    ///
    /// This property defines a custom CSS class that is applied when the option is selected. It enables you to
    /// style the selected option differently, such as changing its background color or text style. Defaults to an
    /// empty string if not provided.
    #[prop_or_default]
    pub selected_class: &'static str,

    /// Inline styles for a selected option.
    ///
    /// This property defines inline styles applied when the option is selected. It provides direct control over
    /// the selected state styling, allowing for unique visual differentiation between selected and non-selected
    /// options. Defaults to an empty string if not provided.
    #[prop_or_default]
    pub selected_style: &'static str,
}

#[function_component(Option)]
pub fn option(props: &OptionProps) -> Html {
    let OptionProps {
        label,
        selected,
        disabled,
        on_click,
        class,
        style,
        selected_style,
        selected_class,
        ..
    } = props.clone();

    let handle_click = {
        let on_click = on_click.clone();
        Callback::from(move |_| on_click.emit(()))
    };

    html! {
        <option
            class={format!("{} {}", class, if selected { selected_class } else { "" })}
            style={format!("{} {}", style, if selected { selected_style } else { "" })}
            onclick={move |ev: MouseEvent| {
                ev.prevent_default();
                handle_click.emit(ev);
            }}
            disabled={disabled}
        >
            { label }
        </option>
    }
}
