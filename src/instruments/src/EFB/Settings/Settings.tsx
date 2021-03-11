/*
 * A32NX
 * Copyright (C) 2020-2021 FlyByWire Simulations and its contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

import React from 'react';
import { Toggle } from '../Components/Form/Toggle';
import { Select, SelectGroup, SelectItem } from '../Components/Form/Select';
import { Slider } from '../Components/Form/Slider';
import { useSimVarSyncedPersistentProperty, usePersistentProperty } from '../../Common/persistence';

const PlaneSettings: React.FC = () => {
    const [adirsAlignTime, setAdirsAlignTime] = usePersistentProperty('CONFIG_ALIGN_TIME', 'REAL');
    // const [dmcSelfTestTime, setDmcSelfTestTime] = usePersistentProperty('CONFIG_SELF_TEST_TIME');
    const [atisSource, setAtisSource] = usePersistentProperty('CONFIG_ATIS_SRC', 'FAA');
    const [metarSource, setMetarSource] = usePersistentProperty('CONFIG_METAR_SRC', 'MSFS');
    const [tafSource, setTafSource] = usePersistentProperty('CONFIG_TAF_SRC', 'NOAA');
    const [paxSigns, setPaxSigns] = usePersistentProperty('CONFIG_USING_PORTABLE_DEVICES', '0');
    const [defaultBaro, setDefaultBaro] = usePersistentProperty('CONFIG_INIT_BARO_UNIT', 'IN HG');

    return (
        <div className="bg-gray-800 rounded-xl px-6 py-4 shadow-lg">
            <h1 className="text-xl font-medium text-white mb-3">Realism</h1>

            <div className="divide-y divide-gray-700 flex flex-col">
                <div className="mb-3.5 flex flex-row justify-between items-center">
                    <span className="text-lg text-gray-300">ADIRS Align Time</span>
                    <SelectGroup>
                        <SelectItem selected={adirsAlignTime === 'INSTANT'} onSelect={() => setAdirsAlignTime('INSTANT')}>Instant</SelectItem>
                        <SelectItem selected={adirsAlignTime === 'FAST'} onSelect={() => setAdirsAlignTime('FAST')}>Fast</SelectItem>
                        <SelectItem selected={adirsAlignTime === 'REAL'} onSelect={() => setAdirsAlignTime('REAL')}>Real</SelectItem>
                    </SelectGroup>
                </div>
                <div className="mb-4 pt-3 opacity-40 flex flex-row justify-between items-center">
                    <span className="text-lg text-gray-300">DMC self-test</span>
                    <SelectGroup>
                        <SelectItem>Instant</SelectItem>
                        <SelectItem>Fast</SelectItem>
                        <SelectItem selected>Real</SelectItem>
                    </SelectGroup>
                </div>
            </div>

            <h1 className="text-xl text-white font-medium mt-4 mb-3">ATSU/AOC</h1>

            <div className="divide-y divide-gray-700 flex flex-col">
                <div className="mb-3.5 flex flex-row justify-between items-center">
                    <span className="text-lg text-gray-300">ATIS Source</span>
                    <SelectGroup>
                        <SelectItem selected={atisSource === 'FAA'} onSelect={() => setAtisSource('FAA')}>FAA (US)</SelectItem>
                        <SelectItem selected={atisSource === 'PILOTEDGE'} onSelect={() => setAtisSource('PILOTEDGE')}>PilotEdge</SelectItem>
                        <SelectItem selected={atisSource === 'IVAO'} onSelect={() => setAtisSource('IVAO')}>IVAO</SelectItem>
                        <SelectItem selected={atisSource === 'VATSIM'} onSelect={() => setAtisSource('VATSIM')}>VATSIM</SelectItem>
                    </SelectGroup>
                </div>
                <div className="mb-3.5 pt-3 flex flex-row justify-between items-center">
                    <span className="text-lg text-gray-300">METAR Source</span>
                    <SelectGroup>
                        <SelectItem selected={metarSource === 'MSFS'} onSelect={() => setMetarSource('MSFS')}>MeteoBlue</SelectItem>
                        <SelectItem selected={metarSource === 'IVAO'} onSelect={() => setMetarSource('IVAO')}>IVAO</SelectItem>
                        <SelectItem selected={metarSource === 'PILOTEDGE'} onSelect={() => setMetarSource('PILOTEDGE')}>PilotEdge</SelectItem>
                        <SelectItem selected={metarSource === 'VATSIM'} onSelect={() => setMetarSource('VATSIM')}>VATSIM</SelectItem>
                    </SelectGroup>
                </div>
                <div className="pt-3 flex flex-row justify-between items-center">
                    <span className="text-lg text-gray-300">TAF Source</span>
                    <SelectGroup>
                        <SelectItem selected={tafSource === 'IVAO'} onSelect={() => setTafSource('IVAO')}>IVAO</SelectItem>
                        <SelectItem selected={tafSource === 'NOAA'} onSelect={() => setTafSource('NOAA')}>NOAA</SelectItem>
                    </SelectGroup>
                </div>
            </div>

            <h1 className="text-xl text-white font-medium mt-5 mb-3">CIDS</h1>

            <div className="pt-3 flex flex-row justify-between items-center">
                <span className="text-lg text-gray-300">PAX Signs</span>
                <SelectGroup>
                    <SelectItem selected={paxSigns === '0'} onSelect={() => setPaxSigns('0')}>No Smoking</SelectItem>
                    <SelectItem selected={paxSigns === '1'} onSelect={() => setPaxSigns('1')}>No Port Devc</SelectItem>
                </SelectGroup>
            </div>

            <h1 className="text-xl text-white font-medium mt-5 mb-3">FMGC</h1>

            <div className="divide-y divide-gray-700 flex flex-col">
                <div className="mb-3.5 opacity-40 flex flex-row justify-between items-center">
                    <span className="text-lg text-gray-300">Thrust Reduction Altitude</span>
                    <div className="flex flex-row">
                        <Select>1500 ft</Select>
                    </div>
                </div>
                <div className="mb-3.5 opacity-40 pt-3 flex flex-row justify-between items-center">
                    <span className="text-lg text-gray-300">Acceleration Altitude </span>
                    <div className="flex flex-row">
                        <Select>1500 ft</Select>
                    </div>
                </div>
                <div className="mb-3.5 opacity-40 pt-3 flex flex-row justify-between items-center">
                    <span className="text-lg text-gray-300">Transition Level</span>
                    <div className="flex flex-row">
                        <Select>FL180</Select>
                    </div>
                </div>

                <div className="w-full pt-2 flex flex-row justify-between">
                    <div className="pt-2 pr-4 flex-grow flex flex-row justify-between items-center">
                        <span className="text-lg text-gray-300">Default Baro</span>
                        <SelectGroup>
                            <SelectItem selected={defaultBaro === 'AUTO'} onSelect={() => setDefaultBaro('AUTO')}>Auto</SelectItem>
                            <SelectItem selected={defaultBaro === 'IN HG'} onSelect={() => setDefaultBaro('IN HG')}>in Hg</SelectItem>
                            <SelectItem selected={defaultBaro === 'HPA'} onSelect={() => setDefaultBaro('HPA')}>hPa</SelectItem>
                        </SelectGroup>
                    </div>
                    <div className="pt-2 pl-4 flex-grow flex flex-row justify-between items-center">
                        <span className="text-lg text-gray-300">Weight Unit</span>
                        <SelectGroup>
                            <SelectItem selected>Kg</SelectItem>
                            <SelectItem>lbs</SelectItem>
                        </SelectGroup>
                    </div>
                </div>
            </div>
        </div>
    );
};

const SoundSettings: React.FC = () => {
    const [ptuAudible, setPtuAudible] = useSimVarSyncedPersistentProperty('L:A32NX_SOUND_PTU_AUDIBLE_COCKPIT', 'Bool', 'SOUND_PTU_AUDIBLE_COCKPIT');
    const [exteriorVolume, setExteriorVolume] = useSimVarSyncedPersistentProperty('L:A32NX_SOUND_EXTERIOR_MASTER', 'number', 'SOUND_EXTERIOR_MASTER');
    const [engineVolume, setEngineVolume] = useSimVarSyncedPersistentProperty('L:A32NX_SOUND_INTERIOR_ENGINE', 'number', 'SOUND_INTERIOR_ENGINE');
    const [windVolume, setWindVolume] = useSimVarSyncedPersistentProperty('L:A32NX_SOUND_INTERIOR_WIND', 'number', 'SOUND_INTERIOR_WIND');

    return (
        <div className="bg-gray-800 rounded-xl px-6 py-4 shadow-lg">
            <div className="divide-y divide-gray-700 flex flex-col">
                <div className="mb-4 flex flex-row justify-between items-center">
                    <span>
                        <span className="text-lg text-gray-300">PTU Audible in Cockpit</span>
                        <span className="text-lg text-gray-500 ml-2">(unrealistic)</span>
                    </span>
                    <Toggle value={!!ptuAudible} onToggle={(value) => setPtuAudible(value ? 1 : 0)} />
                </div>
                <div className="mb-4 pt-3 flex flex-row justify-between items-center">
                    <span className="text-lg text-gray-300">Exterior Master Volume</span>
                    <div className="flex flex-row items-center">
                        <span className="text-base pr-3">{exteriorVolume}</span>
                        <Slider className="w-60" value={exteriorVolume + 50} onInput={(value) => setExteriorVolume(value - 50)} />
                    </div>
                </div>
                <div className="mb-4 pt-3 flex flex-row justify-between items-center">
                    <span className="text-lg text-gray-300">Engine Interior Volume</span>
                    <div className="flex flex-row items-center">
                        <span className="text-base pr-3">{engineVolume}</span>
                        <Slider className="w-60" value={engineVolume + 50} onInput={(value) => setEngineVolume(value - 50)} />
                    </div>
                </div>
                <div className="pt-3 flex flex-row justify-between items-center">
                    <span className="text-lg text-gray-300">Wind Interior Volume</span>
                    <div className="flex flex-row items-center">
                        <span className="text-base pr-3">{windVolume}</span>
                        <Slider className="w-60" value={windVolume + 50} onInput={(value) => setWindVolume(value - 50)} />
                    </div>
                </div>
            </div>
        </div>
    );
};

const FlyPadSettings: React.FC = () => {
    const [brightness, setBrightness] = useSimVarSyncedPersistentProperty('L:A32NX_EFB_BRIGHTNESS', 'number', 'EFB_BRIGHTNESS');

    return (
        <div className="bg-gray-800 divide-y divide-gray-700 flex flex-col rounded-xl px-6 py-4 shadow-lg">
            <div className="flex flex-row justify-between items-center">
                <span className="text-lg text-gray-300">Brightness</span>
                <Slider className="w-60" value={brightness} onInput={(value) => setBrightness(value)} />
            </div>
        </div>
    );
};

const Settings: React.FC = () => (
    <div className="w-full h-full flex flex-col">
        <div className="flex-grow m-6 rounded-xl flex flex-row">
            <div className="w-1/2 pr-3">
                <h1 className="text-2xl text-white mb-4">Plane Settings</h1>
                <PlaneSettings />
            </div>
            <div className="w-1/2 pl-3">
                <h1 className="text-2xl text-white mb-4">Audio Settings</h1>
                <SoundSettings />

                <h1 className="text-2xl text-white mt-5 mb-4">flyPad Settings</h1>
                <FlyPadSettings />

                <h1 className="text-4xl text-center text-gray-700 pt-10">flyPadOS</h1>
                <h1 className="text-xl text-center text-gray-600 py-2">vAlpha</h1>
                <h1 className="text-md text-center text-gray-700 py-2">Copyright &copy; 2020-2021 FlyByWire Simulations</h1>
            </div>
        </div>
    </div>
);

export default Settings;
