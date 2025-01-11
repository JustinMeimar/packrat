use std::io;
use std::io::{Stdout, Write, Read};
use crossterm::execute;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use crate::ui::view::{Transition, View};
use crate::ui::state::SelectionState;
use tui::buffer::Buffer;
use tui::{ 
    backend::{Backend, CrosstermBackend},
    layout::Layout,
    widgets::{List, Table, Widget},
    Terminal,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
///////////////////////////////////////////////////////////

pub type TerminalTy = Terminal<CrosstermBackend<Stdout>>;

///
/// A way to keep a vector of types that implement the trait Widget is to wrap said
/// types in an enum. We use a lifetime parameter because the widget types such as
/// List and Table require so.
#[derive(Clone)]
pub enum AnyWidget<'a> {
    List(List<'a>),
    Table(Table<'a>),
    Paragraph(Paragraph<'a>),
}

pub enum ControlOption {
    T(Transition),
    E(Event),
}

impl<'a> Widget for AnyWidget<'a> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            AnyWidget::List(list) => list.render(area, buf),
            AnyWidget::Table(table) => table.render(area, buf),
            AnyWidget::Paragraph(para) => para.render(area, buf),
        }
    }
}

pub trait Renderable { 
    
    /// Main interface to render a view on a layout 
    fn render(&mut self) -> io::Result<Transition>; 

    /// Refresh dispaly items, default none
    fn update(&mut self) {}
    
    /// Set time interval to trigger updates
    /// TODO: What do I need to do to let a default impl work when it must reference self?
    fn poll(&mut self) {} 
    
    /// Return a division of the current frame into chunks 
    fn chunks(&self, frame: Rect) -> Vec<Rect> { Layout::default().split(frame) }
    
    /// What we actually drawin 
    fn widgets(&mut self) -> io::Result<Vec<AnyWidget>> { Ok(vec![]) } 
 
    // Handle the keyboard controls for a view
    fn controler(&mut self) -> Transition { Transition::Stay }
}

///
/// Try to match one of the default events, in which case return the appropriate
/// transition, otherwise return the event read so it can be dealt with manually
pub fn default_controls(selector: &mut SelectionState) -> ControlOption {
    
    let event: Event = event::read().unwrap();
    match event {

        // Event::Key(KeyEvent { code: KeyCode::Char('q'), .. })
        //     => ControlOption::T(Transition::Push()),

        Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) 
            => ControlOption::T(Transition::Quit), 
        
        Event::Key(KeyEvent { code: KeyCode::Char('b'), .. })
            => ControlOption::T(Transition::Pop),
        
            Event::Key(KeyEvent { code: KeyCode::Char('j') | KeyCode::Down, .. })
            => {
                selector.decr(); ControlOption::T(Transition::Stay)
            }, 
        
        Event::Key(KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, .. })
            => {
                selector.incr();
                ControlOption::T(Transition::Stay)
            },   
        _ 
            => ControlOption::E(event),
    } 
}

///
/// Render a generic View type on the screen,
/// TODO: remove need for control_handler
pub fn render_view<R>(
    state: &mut R,
    control_handler: impl Fn(&mut R) -> Transition,
) -> io::Result<Transition>
where
    R: Renderable,
{
    let mut terminal = render_view_startup()?;    
    let transition = loop {
        state.poll();
        terminal.draw(|f| {
            let chunks = state.chunks(f.size());
            let widgets = state.widgets().unwrap();
            widgets.into_iter().enumerate().for_each(|(i, w)| {
                f.render_widget(w, chunks[i]);
            });
        })?;

        /// For some views, like dialogue boxes that should appear "layered",
        /// we don't want to clear the screen below. Same for Stay transitions,
        /// since doing so induces a flicker.
        let transition = control_handler(state);
        match transition {
            Transition::Stay => {
                return Ok(transition);
            }
            Transition::Push(View::DeleteView(_)) | Transition::Push(View::CreateTaskView(_)) => {
                return Ok(transition);
            }
            _ => {
                terminal.clear()?;
                return Ok(transition);
            }
        } 
    };
    
    terminal.clear()?;
    render_view_teardown(&mut terminal);
    
    Ok(transition)
}

///////////////////////////////////////////////////////////

pub fn render_view_startup() -> io::Result<TerminalTy> { 
    // Flush stdout
    let mut stdout = std::io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;

    Ok(terminal)
}

pub fn render_view_teardown(terminal: &mut TerminalTy) -> io::Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

