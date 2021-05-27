pub struct Region
{
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

impl Region
{
    pub fn new() -> Self
    {
        Self{ xmin: 0.0, xmax: 0.0, ymin: 0.0, ymax: 0.0 }
    }

    pub fn enlarge(&mut self, by: &Region)
    {
        if by.xmin < self.xmin
        {
            self.xmin = by.xmin;
        }
        if by.xmax > self.xmax
        {
            self.xmax = by.xmax;
        }
        if by.ymin < self.ymin
        {
            self.ymin = by.ymin;
        }
        if by.ymax > self.ymax
        {
            self.ymax = by.ymax;
        }
    }

    pub fn extendPositiveX(&mut self, by: f64)
    {
        self.xmax += by;
    }
}
