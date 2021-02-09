
/// Models one “variable” in memory. Could be multiple “bytes”.
#[derive(Clone)]
pub struct Cell
{
    /// The “name” of the “variable”. Shown to the left of the “ladder”.
    pub label: String,
    /// The “value” of the “variable”. Shown inside the “laddder”. For
    /// cells with size > 1, this is displayed in the first “byte”.
    pub content: String,
    /// The “address” of the “variable”. Shown to the right of the
    /// “ladder”.
    pub address: String,
    /// The number of units this cell occupies in the “ladder”.
    pub size: usize,
}

impl Default for Cell
{
    fn default() -> Self
    {
        Self {
            label: String::default(),
            content: String::default(),
            address: String::default(),
            size: 1,
        }
    }
}

impl Cell
{
    /// Whether the address will be displayed or not. Currently the
    /// address is not shown is `address` is empty.
    pub fn showAddress(&self) -> bool
    {
        !self.address.is_empty()
    }
}

/// Models a continuous section of memory. This is usually used to
/// display the scope of a function.
#[derive(Clone)]
pub struct Scope
{
    /// Name of the scope.
    pub name: String,
    pub cells: Vec<Cell>,
}

impl Scope
{
    pub fn new(name: &str) -> Self
    {
        Scope{ name: name.to_owned(), cells: Vec::new() }
    }

    pub fn showName(&self) -> bool
    {
        !self.name.is_empty()
    }

    pub fn size(&self) -> usize
    {
        self.cells.iter().map(|c| c.size).sum()
    }

    /// Add a cell to the scope. The cell will be appended at the
    /// bottom.
    pub fn addCell(&mut self, cell: Cell)
    {
        self.cells.push(cell);
    }

    pub fn findCell(&self, label: &str) -> Option<&Cell>
    {
        self.cells.iter().find(|c| c.label == label)
    }

    pub fn findCellIndex(&self, label: &str) -> Option<usize>
    {
        for i in 0..self.cells.len()
        {
            if self.cells[i].label == label
            {
                return Some(i);
            }
        }
        None
    }

    pub fn cellBegin(&self, index: usize) -> usize
    {
        self.cells[..index].iter().map(|c| c.size).sum()
    }
}

#[derive(Clone, Copy)]
pub struct CellIndex
{
    pub scope: usize,
    pub cell: usize,
}

#[derive(Clone)]
pub struct CellName
{
    pub scope: String,
    pub cell: String,
}

#[derive(Clone)]
pub struct Pointer
{
    pub from: CellName,
    pub to: CellName,
}

impl Pointer
{
    pub fn new(from: (&str, &str), to: (&str, &str)) -> Self
    {
        Self {
            from: CellName{ scope: from.0.to_owned(), cell: from.1.to_owned() },
            to: CellName{ scope: to.0.to_owned(), cell: to.1.to_owned() },
        }
    }
}

pub struct Column
{
    pub scopes: Vec<Scope>,
    pub pointers: Vec<Pointer>,
}

impl Column
{
    pub fn new() -> Self
    {
        Self{ scopes: Vec::new(), pointers: Vec::new() }
    }

    pub fn addScope(&mut self, scope: Scope)
    {
        self.scopes.push(scope);
    }

    pub fn addPointer(&mut self, pointer: Pointer)
    {
        self.pointers.push(pointer);
    }

    pub fn findCell(&self, scope_name: &str, cell_label: &str) -> Option<&Cell>
    {
        for scope in &self.scopes
        {
            if scope.name == scope_name
            {
                return scope.findCell(cell_label);
            }
        }
        None
    }

    pub fn findCellIndex(&self, scope_name: &str, cell_label: &str) ->
        Option<CellIndex>
    {
        for i in 0..self.scopes.len()
        {
            if self.scopes[i].name == scope_name
            {
                if let Some(j) = self.scopes[i].findCellIndex(cell_label)
                {
                    return Some(CellIndex{ scope: i, cell: j });
                }
            }
        }
        None
    }
}
