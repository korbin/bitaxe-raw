use super::CommandError;
use heapless::Vec;

pub struct Pins<'d> {
    pub asic_resetn: esp_hal::gpio::Output<'d>,
}

#[derive(defmt::Format)]
pub enum Command {
    SetAsicResetn { level: bool },
    GetAsicResetn,
}

impl Command {
    pub fn from_bytes(buf: &[u8]) -> Result<Self, CommandError> {
        match buf {
            // Get ASIC Reset (Active Low)
            [0x00] => Ok(Self::GetAsicResetn),
            // Set ASIC Reset (Active Low)
            [0x00, level] => Ok(Self::SetAsicResetn { level: *level > 0 }),
            _ => Err(CommandError::Invalid),
        }
    }
}

impl super::ControllerCommand for Command {
    async fn handle(&self, controller: &mut super::Controller) -> Result<Vec<u8, 256>, CommandError> {
        let level = match self {
            Command::GetAsicResetn => bool::from(controller.gpio.asic_resetn.output_level()),
            Command::SetAsicResetn { level } => {
                controller.gpio.asic_resetn.set_level((*level).into());
                *level
            }
        };

        Ok(Vec::from_slice(&[level as u8]).unwrap())
    }
}
