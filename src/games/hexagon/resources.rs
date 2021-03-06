
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Resource {
    Block,
    Rock,
    Timber,
    Fiber,
    Cereal,
    Desert
}

#[derive(Debug, PartialEq)]
pub struct ResourceList {
    pub block: u16,
    pub rock: u16,
    pub timber: u16,
    pub fiber: u16,
    pub cereal: u16
}

type ResourceArray<const N: usize> = [Resource; N];

impl ResourceList {
    pub fn new() -> ResourceList {
        ResourceList { block: 0, rock: 0, timber: 0, fiber: 0, cereal: 0 }
    }

    pub fn to_array(&self) -> [(Resource, u16); 5] {
        [
            (Resource::Block, self.block),
            (Resource::Rock, self.rock),
            (Resource::Timber, self.timber),
            (Resource::Fiber, self.fiber),
            (Resource::Cereal, self.cereal)
        ]
    }

    pub fn count(&self) -> u16 {
        self.block + self.rock + self.timber + self.fiber + self.cereal
    }

    pub fn deposit<const N: usize>(&mut self, resources: ResourceArray<N>) -> Result<(),&'static str> {
        for resource in resources {
            match resource {
                Resource::Block => self.block += 1,
                Resource::Rock => self.rock += 1,
                Resource::Timber => self.timber += 1,
                Resource::Fiber => self.fiber += 1,
                Resource::Cereal => self.cereal += 1,
                Resource::Desert => return Err("Can't deposit Desert resources.")
            }
        }

        Ok(())
    }

    pub fn deduct<const N: usize>(&mut self, resources: ResourceArray<N>) -> Result<(),&'static str> {
        for resource in resources {
            match resource {
                Resource::Block => 
                    if self.block > 0 { self.block -= 1 } 
                    else { return Err("Can't deduct; not enough resources.") },
                Resource::Rock => 
                    if self.rock > 0 { self.rock -= 1 } 
                    else { return Err("Can't deduct; not enough resources.") },
                Resource::Timber => 
                    if self.timber > 0 { self.timber -= 1 } 
                    else { return Err("Can't deduct; not enough resources.") },
                Resource::Fiber => 
                    if self.fiber > 0 { self.fiber -= 1 } 
                    else { return Err("Can't deduct; not enough resources.") },
                Resource::Cereal => 
                    if self.cereal > 0 { self.cereal -= 1 } 
                    else { return Err("Can't deduct; not enough resources.") },
                Resource::Desert => 
                    return Err("Can't deduct Desert resources.")
            }
        }

        Ok(())
    }

    pub fn check<const N: usize>(&mut self, resources: ResourceArray<N>) -> Result<(),&'static str> {

        let mut the_bill = ResourceList::new();
        let _status = the_bill.deposit(resources);
        let bill_array = the_bill.to_array();
        
        let mut can_pay_the_bill = true;
        for (rsrc,amnt) in bill_array {
            match rsrc {
                Resource::Block => if self.block < amnt { can_pay_the_bill = false },
                Resource::Rock => if self.rock < amnt { can_pay_the_bill = false },
                Resource::Timber => if self.timber < amnt { can_pay_the_bill = false },
                Resource::Fiber => if self.fiber < amnt { can_pay_the_bill = false },
                Resource::Cereal => if self.cereal < amnt { can_pay_the_bill = false },
                Resource::Desert => ()
            }
        }

        if can_pay_the_bill { Ok(()) }
        else { Err("Not enough resources to build.") }
    }
}

#[cfg(test)]

#[test]
fn resource_lists() {
    let mut resource_list = ResourceList::new();
    assert_eq!(
        (resource_list.block, resource_list.rock, resource_list.timber, resource_list.fiber, resource_list. cereal),
        (0, 0, 0, 0, 0)
    );

    let _status = resource_list.deposit([Resource::Block]);
    assert_eq!(
        (resource_list.block, resource_list.rock, resource_list.timber, resource_list.fiber, resource_list. cereal),
        (1, 0, 0, 0, 0)
    );

    let _status = resource_list.deposit([Resource::Rock, Resource::Timber, Resource::Fiber, Resource::Cereal]);
    assert_eq!(
        (resource_list.block, resource_list.rock, resource_list.timber, resource_list.fiber, resource_list. cereal),
        (1, 1, 1, 1, 1)
    );

    let _status = resource_list.deduct([Resource::Block, Resource::Rock, Resource::Timber]);
    assert_eq!(
        (resource_list.block, resource_list.rock, resource_list.timber, resource_list.fiber, resource_list. cereal),
        (0, 0, 0, 1, 1)
    );

}

#[test]
fn resource_list_errors() {
    let mut resource_list = ResourceList::new();
    let attempt = resource_list.deposit([Resource::Desert]);
    assert_eq!(attempt, Err("Can't deposit Desert resources."));
    let attempt = resource_list.deduct([Resource::Desert]);
    assert_eq!(attempt, Err("Can't deduct Desert resources."));
}

#[test]
fn credit_check() {
    let mut resource_list = ResourceList::new();
    let check = resource_list.check([Resource::Block, Resource::Block, Resource::Timber]);
    assert_eq!(check, Err("Not enough resources to build."));
    let _status = resource_list.deposit([Resource::Block, Resource::Timber]);
    let check = resource_list.check([Resource::Block, Resource::Block, Resource::Timber]);
    assert_eq!(check, Err("Not enough resources to build."));
    let _status = resource_list.deposit([Resource::Block]);
    let check = resource_list.check([Resource::Block, Resource::Block, Resource::Timber]);
    assert_eq!(check, Ok(()));
    let _status = resource_list.deposit([Resource::Block, Resource::Timber]);
    let check = resource_list.check([Resource::Block, Resource::Block, Resource::Timber]);
    assert_eq!(check, Ok(()));
}