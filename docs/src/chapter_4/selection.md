# Selection

Single choice selection.

### Usage
```rust
let selection = selection!(
    placeholder: "Select choice",
    model: "select",
    choices: ["choice 1", "choice 2"]
);
```
Return `Entity` which must be used inside a containable widget.

### Example
```rust
fa_query.insert_str("plan", "");
fa_query.insert_str("subscription", "");

let plans = selection!(
    placeholder: "Select plan",
    model: "plan",
    choices: ["Personal", "Team", "Enterprise"]
);
let subscriptions = selection!(
    placeholder: "Select subscription payment",
    model: "subscription",
    choices: ["Weekly", "Monthly", "Annually"]
);
container!(children: [plans, subscriptions]);
```

### Required attributes
- **placeholder**
- **model**: type string.

### Available attributes
- **id**
- **class**
- **color**
- **choices**
