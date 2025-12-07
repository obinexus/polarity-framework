import re
from enum import Enum
from dataclasses import dataclass
from typing import Union, Tuple, Optional

class State(Enum):
    STABLE = "stable"
    EXPERIMENTAL = "experimental"
    LEGACY = "legacy"

    def __str__(self):
        return self.value

    def __lt__(self, other):
        # Define hierarchy: Legacy < Experimental < Stable
        # This relates to the "Scoring" mentioned in your transcript
        order = {State.LEGACY: 0, State.EXPERIMENTAL: 1, State.STABLE: 2}
        if not isinstance(other, State):
            return NotImplemented
        return order[self] < order[other]

@dataclass
class VersionComponent:
    value: int
    state: State = State.STABLE

    def __str__(self):
        return f"{self.value}.{self.state}"

class SemVerX:
    """
    Semantic Version Extended (SemVerX)
    Implementation of the schema:
    Major.[Stable|Experimental|Legacy]
    Minor.[Stable|Experimental|Legacy]
    Patch.[Stable|Experimental|Legacy]
    """
    def __init__(
        self, 
        major: Union[int, Tuple[int, str]], 
        minor: Union[int, Tuple[int, str]], 
        patch: Union[int, Tuple[int, str]]
    ):
        self.major = self._parse_component(major)
        self.minor = self._parse_component(minor)
        self.patch = self._parse_component(patch)

    def _parse_component(self, comp) -> VersionComponent:
        if isinstance(comp, int):
            return VersionComponent(comp, State.STABLE)
        elif isinstance(comp, tuple) and len(comp) == 2:
            try:
                state_str = comp[1].lower().strip()
                # Map various inputs to the Enum
                if "exp" in state_str: state = State.EXPERIMENTAL
                elif "leg" in state_str: state = State.LEGACY
                else: state = State.STABLE
                return VersionComponent(comp[0], state)
            except Exception:
                return VersionComponent(comp[0], State.STABLE)
        elif isinstance(comp, VersionComponent):
            return comp
        raise ValueError(f"Invalid component format: {comp}")

    def to_string(self) -> str:
        """Returns the full extended version string."""
        return f"{self.major.value}.{self.major.state}.{self.minor.value}.{self.minor.state}.{self.patch.value}.{self.patch.state}"

    def get_coherence_score(self) -> float:
        """
        Calculates a 'Quality Assurance' score based on component states.
        (Referencing the A* Scoring / Matrix logic in the transcript)
        Stable = 1.0, Experimental = 0.5, Legacy = 0.1
        """
        weights = {State.STABLE: 1.0, State.EXPERIMENTAL: 0.5, State.LEGACY: 0.1}
        
        score = (weights[self.major.state] * 3.0 + 
                 weights[self.minor.state] * 1.5 + 
                 weights[self.patch.state] * 0.5)
        
        # Max score is 3.0 + 1.5 + 0.5 = 5.0. Normalize to 0-1
        return round(score / 5.0, 3)

    def __repr__(self):
        return f"<SemVerX {self.to_string()}>"

def semver_version_extended(
    major_config: str, 
    minor_config: str, 
    patch_config: str
) -> SemVerX:
    """
    The main function requested to parse the specific extended format.
    
    Accepts strings in format: "1.stable", "2.experimental", etc.
    
    Args:
        major_config: e.g., "1.stable" or "1.legacy"
        minor_config: e.g., "0.experimental"
        patch_config: e.g., "4.stable"
    """
    
    def parse_str(s):
        parts = s.split('.')
        val = int(parts[0])
        state = parts[1] if len(parts) > 1 else "stable"
        return (val, state)

    return SemVerX(
        parse_str(major_config),
        parse_str(minor_config),
        parse_str(patch_config)
    )

# --- Demonstration of Usage based on your transcript ---

if __name__ == "__main__":
    print("--- Semantic Version X System ---")

    # 1. Creating a version using the function requested
    # "via major.stable, major.exprimental..."
    v1 = semver_version_extended("1.stable", "4.experimental", "2.stable")
    print(f"Version 1 Created: {v1.to_string()}")
    print(f"Coherence Score: {v1.get_coherence_score()} (Mixed stability)")

    print("\n--- Migration/Legacy Test ---")
