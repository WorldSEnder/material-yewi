use material_yewi::app_bar::{AppBar, AppBarPosition};
use material_yewi::button::{Button, ButtonColor};
use material_yewi::toolbar::Toolbar;
use material_yewi::typography::{Typography, TypographyVariant};
use stylist::ast::sheet;
use yew::{html, Html};
use yew_feather::menu::Menu;

pub fn render() -> Html {
    html! {
        // FIXME: Absolute position necessary cause we don't iframe in demos correctly
        <AppBar position={AppBarPosition::Static}>
            <Toolbar>
                <Button color={ButtonColor::Inherit}>
                    <Menu />
                </Button>
                <Typography class={sheet!(flex-grow: 1;)} variant={TypographyVariant::H6}>
                    {"News"}
                </Typography>
                <Button color={ButtonColor::Inherit}>
                    {"Login"}
                </Button>
            </Toolbar>
        </AppBar>
    }
}
