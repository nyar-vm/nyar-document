use super::*;

#[derive(Clone, Debug, PartialEq, Props)]
pub struct PackageContext {
    package: String,
    version: String,
    language: String,
    theme: String,
    platform: String,
}

#[derive(Debug, Deserialize)]
pub struct PackageQuery {
    #[serde(default)]
    version: String,
    #[serde(default)]
    language: String,
    #[serde(default)]
    theme: String,
    #[serde(default)]
    platform: String,
}

impl PackageContext {
    pub fn new(package: String) -> Self {
        Self { package, version: "".to_string(), language: "".to_string(), theme: "".to_string(), platform: "".to_string() }
    }
    pub fn as_module_link(&self) -> LazyNodes {
        let pkg = self.package.as_str();
        let query = if self.version.is_empty() { "?version=latest".to_string() } else { format!("?version={}", self.version) };
        // if !self.language.is_empty() {
        //     query.push_str(&format!("&language={}", self.language));
        // }
        rsx! {
            a {
                href: "{pkg}/{pkg}/{query}",
                "{pkg}"
            }
        }
    }
}

impl Add<PackageQuery> for PackageContext {
    type Output = Self;

    fn add(self, rhs: PackageQuery) -> Self::Output {
        Self { version: rhs.version, language: rhs.language, theme: rhs.theme, platform: rhs.platform, ..self }
    }
}

// create a component that renders a div with the text "hello world"
pub fn PackagePage(cx: Scope<PackageContext>) -> Element {
    let pkg = cx.props.package.as_str();
    let version = cx.props.version.as_str();
    let language = cx.props.language.as_str();
    // let module = cx.props.modules.join("::");
    let container = css!(
        "
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 4px;
    "
    );
    cx.render(rsx!(div {
        class: "{container}",
        "hello package"
        div {
            cx.props.as_module_link()
        }
        div {
            "version: {version}"
        }
        div {
            "language: {language}"
        }
    }))
}
