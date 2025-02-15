use std::io;
use std::io::Cursor;

struct DataModel<'a> {
    listeners: Vec<Box<dyn FnMut() + 'a>>,
    num: u8,
}

impl<'a> DataModel<'a> {
    fn add_listener(&mut self, l: impl FnMut() + 'a) {
        self.listeners.push(Box::new(l));
    }

    fn set_num(&mut self, new_num: u8) {
        self.num = new_num;
        for l in self.listeners.iter_mut() {
            l();
        }
    }
}

struct ViewModel {
    repeat: u8,
}
impl ViewModel {
    fn dm_changed(&mut self) {
        println!("IN VM CB");
        self.repeat += 1;
    }

    fn num(&self, dm: &DataModel) -> String {
        let mut n: String = String::new();

        for _ in 0..self.repeat as usize {
            n += format!("{}", dm.num).as_str();
        }

        n
    }
}

struct View {}
impl View {
    fn render<W: io::Write>(&self, w: &mut W, vm: &ViewModel, dm: &DataModel) {
        write!(w, "[{}]", vm.num(&dm)).unwrap();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn exploration_test() {
        let mut buffer = Cursor::new(Vec::new());

        let view: View = View {};
        let mut vm: ViewModel = ViewModel { repeat: 2 };
        let mut dm: DataModel = DataModel {
            num: 23,
            listeners: vec![],
        };

        view.render(&mut buffer, &vm, &dm);
        dm.add_listener(|| ViewModel::dm_changed(&mut vm));
        dm.set_num(23);

        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert_eq!(output, "[2323]");
    }
}
