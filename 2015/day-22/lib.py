class Entity(dict):
    '''Represent a damagable entity such as the player or a boss'''

    def __init__(self, **kwargs):
        for key, val in kwargs.items():
            self[key] = val

    def __getitem__(self, key):
        try:
            return dict.__getitem__(self, key)
        except:
            return 0

    def damage(self, points):
        '''Apply damage to this entity; minimum damage is always 1'''

        self['Hit Points'] -= max(1, points - self['Armor'])

    def tick_active_spells(self, target):
        '''Apply all active spells to the target, remove any that have expired.'''

        if self['Active Spells']:
            for spell in list(self['Active Spells']):
                self['History'].append(str(spell))
                spell.tick(self, target)
                spell.Duration -= 1
                if spell.Duration <= 0:
                    self['History'].append('{} fades'.format(spell.__class__.__name__))
                    spell.fade(self, target)
                    self['Active Spells'].remove(spell)

    def __hash__(self):
        return hash((
            self['Hit Points'],
            self['Mana Points'],
            self['Mana Spent'],
            tuple(self['Active Spells'] or set()),
        ))

class Spell(dict):
    '''
    Create a spell. Spells have `Cost` mana and last `Duration` turns.

    cast() is called when a spell is first cast
    tick() is called each turn (for Duration > 0)
    fade() is called when a spell runs out of duration
    '''


    Cost = float("inf")
    Duration = 0

    def __init__(self):
        self['Duration'] = self.__class__.Duration

    def cast(self, caster, target):
        pass

    def tick(self, caster, target):
        pass

    def fade(self, caster, target):
        pass

    def __repr__(self):
        return '{}({})'.format(self.__class__.__name__, self.Duration)

    def __eq__(self, other):
        return self.__class__.__name__ == other.__class__.__name__

    def __hash__(self):
        return hash(self.__class__.__name__)

class MagicMissle(Spell):
    Cost = 53

    def cast(self, caster, target):
        target.damage(4)

class Drain(Spell):
    Cost = 73

    def cast(self, caster, target):
        target.damage(2)
        caster['Hit Points'] += 2

class Shield(Spell):
    Cost = 113
    Duration = 6

    def cast(self, caster, target):
        caster['Armor'] += 7

    def fade(self, caster, target):
        caster['Armor'] -= 7

class Poison(Spell):
    Cost = 173
    Duration = 6

    def tick(self, caster, target):
        target.damage(3)

class Recharge(Spell):
    Cost = 229
    Duration = 5

    def tick(self, caster, target):
        caster['Mana Points'] += 101

spells = [MagicMissle, Drain, Shield, Poison, Recharge]

class HardMode(Spell):
    Duration = float('inf')

    def tick(self, caster, target):
        self.toggle = not getattr(self, 'toggle', False)
        if self.toggle:
            caster.damage(1)
