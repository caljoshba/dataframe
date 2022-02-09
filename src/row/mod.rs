pub mod types;

use std::rc::{ Rc, Weak };
use std::cell::{
    RefCell,
    Ref
};
use crate::cell::{
    RcCell,
    AnyTypeCell
};

#[derive(Debug)]
pub struct Row {
    cells: RefCell<Vec<Weak<AnyTypeCell>>>,
    pub index: usize,
}

impl Row {
    pub fn new(index: usize) -> RcRow {
        Rc::new(RefCell::new(Self {
            cells: RefCell::new(vec![]),
            index
        }))
    }

    pub fn add_cell(&mut self, cell: &RcCell) {
        self.cells.borrow_mut().push(Rc::downgrade(cell));
    }

    pub fn drop_cell(&mut self, cell: &RcCell) {
        self.cells.borrow_mut().retain(|c| c.upgrade().unwrap() != Rc::clone(cell));
    }

    pub fn get_cells(&self) -> Ref<Vec<Weak<AnyTypeCell>>> {
        self.cells.borrow()
    }

    pub fn get_cell(&self, cell_index: usize) -> Option<RcCell> {
        self.cells.borrow()[cell_index].upgrade()
    }

    pub fn get_last_cell(&self) -> Option<RcCell> {
        let last_cell = self.cells.borrow().clone();
        if let Some(cell) = last_cell.iter().last() {
            return cell.upgrade();
        }
        None
    }

    pub fn update_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl Eq for Row {}

pub type RcRow = Rc<RefCell<Row>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::{
        types::datatypes::AnyType,
        Cell,
        RcCell,
    };
    #[test]
    fn add_cell() {
        let value: AnyType = 67u16.into();
        let row: RcRow = Row::new(3);
        let cell: RcCell = Cell::new(value, &row, "timmeh");

        row.borrow_mut().add_cell(&cell);

        assert!(cell == row.borrow().get_cells()[0].upgrade().unwrap());
    }

    #[test]
    fn add_cell_ref_count() {
        let value: AnyType = 67u16.into();
        let row: RcRow = Row::new(3);
        let cell: RcCell = Cell::new(value, &row, "timmeh");

        row.borrow_mut().add_cell(&cell);

        assert!(1 == row.borrow().get_cells()[0].strong_count());
        assert!(1 == row.borrow().get_cells()[0].weak_count());
    }

    #[test]
    fn drop_cell() {
        let value: AnyType = 67u16.into();
        let row: RcRow = Row::new(3);
        let cell: RcCell = Cell::new(value, &row, "timmeh");

        row.borrow_mut().add_cell(&cell);
        assert!(row.borrow().get_cells().len() == 1);

        row.borrow_mut().drop_cell(&cell);
        assert!(row.borrow().get_cells().len() == 0);
    }

    #[test]
    fn update_index() {
        let value: AnyType = 67u16.into();
        let row: RcRow = Row::new(3);
        let cell: RcCell = Cell::new(value, &row, "timmeh");

        row.borrow_mut().add_cell(&cell);
        row.borrow_mut().update_index(7);
        assert!(row.borrow().index == 7);
    }

}