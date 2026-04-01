use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderFormStatus {
    Draft,
    Cancelled,
    Completed,
    Processing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderFormTenantStatus {
    System(OrderFormStatus),
    Custom {
        id: Uuid,
        tenant_id: Uuid,
        name: String,
        core_status: OrderFormStatus,
    },
}

impl OrderFormTenantStatus {
    pub fn core_status(&self) -> OrderFormStatus {
        match self {
            OrderFormTenantStatus::System(core) => *core,
            OrderFormTenantStatus::Custom { core_status, .. } => *core_status,
        }
    }

    pub fn tenant_id(&self) -> Option<Uuid> {
        match self {
            OrderFormTenantStatus::System(_) => None,
            OrderFormTenantStatus::Custom { tenant_id, .. } => Some(*tenant_id),
        }
    }

    pub fn name(&self) -> String {
        match self {
            OrderFormTenantStatus::System(core) => format!("{:?}", core),
            OrderFormTenantStatus::Custom { name, .. } => name.clone(),
        }
    }

    pub fn new(
        id: Uuid,
        tenant_id: Uuid,
        name: &str,
        core_status: OrderFormStatus,
    ) -> Result<Self, &'static str> {
        if name.trim().is_empty() {
            return Err("Statü adı boş olamaz.");
        }
        Ok(OrderFormTenantStatus::Custom {
            id,
            tenant_id,
            name: name.to_string(),
            core_status,
        })
    }
}

pub struct OrderForm {
    pub id: Uuid,
    pub tenant_id: Uuid,
    status: OrderFormTenantStatus,
}

impl OrderForm {
    pub fn new(
        id: Uuid,
        tenant_id: Uuid,
        initial_status: OrderFormTenantStatus,
    ) -> Result<Self, &'static str> {
        if initial_status.core_status() != OrderFormStatus::Draft {
            return Err("Sipariş sadece Draft statüsü ile başlayabilir.");
        }

        Ok(Self {
            id,
            tenant_id,
            status: initial_status,
        })
    }

    pub fn status(&self) -> &OrderFormTenantStatus {
        &self.status
    }

    pub fn update_status(&mut self, new_status: OrderFormTenantStatus) -> Result<(), &'static str> {
        if let Some(status_tenant_id) = new_status.tenant_id() {
            if status_tenant_id != self.tenant_id {
                return Err("Farklı bir firmaya ait statü bu siparişe atanamaz.");
            }
        }

        let current_core = self.status.core_status();
        let new_core = new_status.core_status();

        if current_core == OrderFormStatus::Draft && new_core != OrderFormStatus::Processing {
            return Err("Draft statüsü sadece Processing'e geçebilir.");
        }

        if current_core == OrderFormStatus::Processing && new_core == OrderFormStatus::Draft {
            return Err("Processing statüsü tekrar Draft'a dönemez.");
        }

        self.status = new_status;
        Ok(())
    }
}

fn main() -> Result<(), &'static str> {
    let firm_tenant_id = Uuid::new_v4();

    let waiting_for_parts = OrderFormTenantStatus::new(
        Uuid::new_v4(),
        firm_tenant_id,
        "Waiting for Parts",
        OrderFormStatus::Processing,
    )?;
    let assembly_line = OrderFormTenantStatus::new(
        Uuid::new_v4(),
        firm_tenant_id,
        "Assembly Line",
        OrderFormStatus::Processing,
    )?;
    let in_tailor = OrderFormTenantStatus::new(
        Uuid::new_v4(),
        firm_tenant_id,
        "In Tailor",
        OrderFormStatus::Processing,
    )?;

    let mut order = OrderForm::new(
        Uuid::new_v4(),
        firm_tenant_id,
        OrderFormTenantStatus::System(OrderFormStatus::Draft),
    )?;

    println!("Initial Order Status: {}", order.status().name());

    order.update_status(waiting_for_parts)?;
    println!("Updated Order Status: {}", order.status().name());

    order.update_status(assembly_line)?;
    println!("Updated Order Status: {}", order.status().name());

    order.update_status(in_tailor)?;
    println!("Updated Order Status: {}", order.status().name());

    order.update_status(OrderFormTenantStatus::System(OrderFormStatus::Cancelled))?;
    println!("Updated Order Status: {}", order.status().name());

    Ok(())
}
