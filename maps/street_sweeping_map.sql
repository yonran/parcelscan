-- Generate Interactive Street Sweeping Frequency Map
-- Run with: duckdb :memory: < street_sweeping_map.sql > street_sweeping_map.html
--
-- This script creates an interactive Leaflet map showing street sweeping frequency
-- across San Francisco, color-coded by how many times per month each street is swept.

.read create_table/street_sweeping.sql

-- Generate HTML map with embedded data
.mode list
.separator ''
.headers off

-- HTML header and Leaflet setup
WITH source_meta AS (
    SELECT
        CAST(rowsUpdatedAt AS BIGINT) AS rows_updated_at
    FROM read_json_auto(street_sweeping_metadata_file())
)
SELECT '<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>SF Street Sweeping Frequency Map</title>
    <!--
        rowsUpdatedAt=' || rows_updated_at || '
    -->
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
    <link rel="stylesheet" href="https://unpkg.com/maplibre-gl/dist/maplibre-gl.css" />
    <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
    <script src="https://unpkg.com/maplibre-gl/dist/maplibre-gl.js"></script>
    <script src="https://unpkg.com/@maplibre/maplibre-gl-leaflet/leaflet-maplibre-gl.js"></script>
    <style>
        body { margin: 0; padding: 0; }
        #map { position: absolute; top: 0; bottom: 0; width: 100%; }
        .legend {
            background: white;
            padding: 10px;
            border: 2px solid #ccc;
            border-radius: 5px;
            font-family: Arial, sans-serif;
            font-size: 14px;
        }
        .legend h4 {
            margin: 0 0 10px 0;
            font-size: 16px;
        }
        .legend-item {
            margin: 5px 0;
        }
        .legend-color {
            display: inline-block;
            width: 30px;
            height: 15px;
            margin-right: 8px;
            vertical-align: middle;
        }
        .map-header {
            position: absolute;
            top: 10px;
            left: 10px;
            z-index: 1000;
            max-width: 420px;
            background: rgba(255, 255, 255, 0.92);
            padding: 12px 14px;
            border: 2px solid #ccc;
            border-radius: 6px;
            font-family: Arial, sans-serif;
            font-size: 13px;
            line-height: 1.3;
        }
        .map-header h1 {
            margin: 0 0 6px 0;
            font-size: 18px;
        }
        .map-header p {
            margin: 6px 0;
        }
        .map-header a {
            color: #1a4b8c;
            text-decoration: none;
        }
        .map-header a:hover {
            text-decoration: underline;
        }
    </style>
</head>
<body>
    <div class="map-header">
        <h1>San Francisco Street Sweeping Frequency</h1>
        <p>Segments are colored by the highest sweeping frequency across either side of the street. If sides differ, the frequency line lists each side separately. Uncolored streets indicate either no sweeping schedule (e.g., Panorama Dr, Miraloma Dr) or no parking allowed (e.g., Sunset Blvd).</p>
        <p>Updated: ' || COALESCE(
            strftime(
                to_timestamp(rows_updated_at) AT TIME ZONE 'America/Los_Angeles',
                '%Y-%m-%d'
            ),
            'unknown'
        ) || '.</p>
        <p>Source: <a href="https://data.sfgov.org/City-Infrastructure/Street-Sweeping-Schedule/yhqp-riqs">SF Open Data - Street Sweeping Schedule</a>.</p>
    </div>
    <div id="map"></div>
    <script>
        var map = L.map("map").setView([37.7749, -122.4194], 12);

        L.maplibreGL({
            style: "https://tiles.openfreemap.org/styles/liberty"
        }).addTo(map);
        map.attributionControl.addAttribution("<a href=\"https://openfreemap.org\" target=\"_blank\">OpenFreeMap</a> <a href=\"https://www.openmaptiles.org/\" target=\"_blank\">© OpenMapTiles</a> Data from <a href=\"https://www.openstreetmap.org/copyright\" target=\"_blank\">OpenStreetMap</a>");

        function getColor(daysPerWeek, timesPerMonth) {
            if (daysPerWeek >= 7) return "#67001f";  // Dark red: Daily
            if (daysPerWeek >= 5) return "#d73027";  // Red: 5-6x per week
            if (daysPerWeek >= 3) return "#fc8d59";  // Orange: 3-4x per week
            if (daysPerWeek === 2) return "#fee090"; // Yellow: 2x per week
            if (daysPerWeek === 1) {
                // One day per week - check monthly frequency
                if (timesPerMonth >= 4) return "#91bfdb";  // Light blue: Weekly (4-5x/month)
                if (timesPerMonth === 3) return "#abd9e9";  // Lighter blue: 3x per month
                if (timesPerMonth === 2) return "#e0f3f8";  // Very light blue: Biweekly
                if (timesPerMonth === 1) return "#ffffbf";  // Pale yellow: Monthly
            }
            if (daysPerWeek === 0) {
                if (timesPerMonth >= 4) return "#91bfdb";  // Light blue: 4+ per month
                if (timesPerMonth === 3) return "#abd9e9";  // Lighter blue: 3x per month
                if (timesPerMonth === 2) return "#e0f3f8";  // Very light blue: 2x per month
                if (timesPerMonth === 1) return "#ffffbf";  // Pale yellow: Monthly
            }
            return "#4575b4";  // Dark blue: Less than weekly
        }

        function getFrequencyLabel(daysPerWeek, timesPerMonth) {
            if (daysPerWeek >= 7) return "Daily (7x/week)";
            if (daysPerWeek === 6) return "6x per week";
            if (daysPerWeek === 5) return "5x per week";
            if (daysPerWeek === 4) return "4x per week";
            if (daysPerWeek === 3) return "3x per week";
            if (daysPerWeek === 2) return "2x per week";
            if (daysPerWeek === 1) {
                // One day per week - specify monthly frequency
                if (timesPerMonth >= 4) return "Weekly (" + timesPerMonth + "x/month)";
                if (timesPerMonth === 3) return "3x per month";
                if (timesPerMonth === 2) return "Biweekly (2x/month)";
                if (timesPerMonth === 1) return "Monthly";
            }
            if (daysPerWeek === 0) {
                if (timesPerMonth >= 4) return timesPerMonth + "x per month";
                if (timesPerMonth === 3) return "3x per month";
                if (timesPerMonth === 2) return "2x per month";
                if (timesPerMonth === 1) return "Monthly";
                return "Less than monthly";
            }
            return "Less than weekly";
        }

        var dayOrder = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
        var dayBits = [1, 2, 4, 8, 16, 32, 64];
        var weekBits = [1, 2, 4, 8, 16];
        var dayFull = {
            Mon: "Monday",
            Tue: "Tuesday",
            Wed: "Wednesday",
            Thu: "Thursday",
            Fri: "Friday",
            Sat: "Saturday",
            Sun: "Sunday"
        };
        var dayShort = {
            Mon: "Mon",
            Tue: "Tue",
            Wed: "Wed",
            Thu: "Thu",
            Fri: "Fri",
            Sat: "Sat",
            Sun: "Sun"
        };

        function joinWithAnd(items) {
            if (!items || items.length === 0) return "";
            if (items.length === 1) return items[0];
            if (items.length === 2) return items[0] + " and " + items[1];
            return items.slice(0, -1).join(", ") + ", and " + items[items.length - 1];
        }

        function formatHour(hour) {
            var h = Number(hour);
            if (h === 0) return "12:01 a.m.";
            if (h === 12) return "12 noon";
            if (h < 12) return h + " a.m.";
            return (h - 12) + " p.m.";
        }

        function formatTimeRange(fromHour, toHour) {
            return formatHour(fromHour) + " to " + formatHour(toHour);
        }

        function formatDayList(days) {
            if (!days || days.length === 0) return "";
            var sorted = days.slice().sort(function(a, b) {
                return dayOrder.indexOf(a) - dayOrder.indexOf(b);
            });
            if (sorted.length === 7) return "every day";
            var ranges = [];
            var start = sorted[0];
            var prev = sorted[0];
            for (var i = 1; i < sorted.length; i++) {
                var cur = sorted[i];
                if (dayOrder.indexOf(cur) === dayOrder.indexOf(prev) + 1) {
                    prev = cur;
                } else {
                    ranges.push([start, prev]);
                    start = cur;
                    prev = cur;
                }
            }
            ranges.push([start, prev]);
            var useShort = sorted.length > 1;
            var labels = ranges.map(function(range) {
                var s = range[0];
                var e = range[1];
                if (s === e) {
                    return useShort ? dayShort[s] : dayFull[s];
                }
                return dayShort[s] + " thru " + dayShort[e];
            });
            return joinWithAnd(labels);
        }

        function formatWeeks(weeks) {
            if (!weeks || weeks.length === 0) return "";
            if (weeks.length === 5) return "";
            var ordinals = weeks.map(function(w) {
                if (w === 1) return "1st";
                if (w === 2) return "2nd";
                if (w === 3) return "3rd";
                if (w === 4) return "4th";
                return "5th";
            });
            return joinWithAnd(ordinals);
        }

        function formatScheduleText(sched) {
            var timeText = formatTimeRange(sched.fromHour, sched.toHour);
            var weekText = formatWeeks(sched.weeks);
            var dayText = "";
            if (sched.days && sched.days.length > 0) {
                dayText = formatDayList(sched.days);
            } else if (sched.hasHolidayDay) {
                dayText = "holidays";
            }
            var text = timeText;
            if (weekText) {
                if (dayText) {
                    text += " " + weekText + " " + dayText + " of the month";
                } else {
                    text += " " + weekText + " of the month";
                }
            } else if (dayText) {
                text += " " + dayText;
            }
            if (dayText && dayText !== "holidays" && (sched.hasHolidayDay || sched.includesHolidays)) {
                text += " including holidays";
            }
            return text;
        }

        function daysFromMask(mask) {
            var days = [];
            for (var i = 0; i < dayOrder.length; i++) {
                if (mask & dayBits[i]) {
                    days.push(dayOrder[i]);
                }
            }
            return days;
        }

        function weeksFromMask(mask) {
            var weeks = [];
            for (var i = 0; i < weekBits.length; i++) {
                if (mask & weekBits[i]) {
                    weeks.push(i + 1);
                }
            }
            return weeks;
        }

        function decodeScheduleDetails(details) {
            if (!details || details.length === 0) return [];
            return details.map(function(item) {
                return {
                    side: item[0],
                    fromHour: item[1],
                    toHour: item[2],
                    days: daysFromMask(item[3] || 0),
                    weeks: weeksFromMask(item[4] || 0),
                    includesHolidays: !!item[5],
                    hasHolidayDay: !!item[6]
                };
            });
        }

        function computeFrequency(scheduleDetails) {
            if (!scheduleDetails || scheduleDetails.length === 0) {
                return {daysPerWeek: 0, timesPerMonth: 0, label: "Less than monthly"};
            }

            var bySide = {};
            scheduleDetails.forEach(function(sched) {
                var side = sched.side || "";
                if (!bySide[side]) {
                    bySide[side] = [];
                }
                bySide[side].push(sched);
            });

            var sideFreqs = [];
            Object.keys(bySide).forEach(function(side) {
                var weeklyDays = 0;
                var timesPerMonth = 0;
                bySide[side].forEach(function(sched) {
                    if (!sched.days || sched.days.length === 0) {
                        return;
                    }
                    var daysCount = sched.days.length;
                    var weeksCount = sched.weeks ? sched.weeks.length : 0;
                    timesPerMonth += daysCount * weeksCount;
                    if (weeksCount === 5) {
                        weeklyDays += daysCount;
                    }
                });
                sideFreqs.push({
                    side: side,
                    daysPerWeek: weeklyDays,
                    timesPerMonth: timesPerMonth,
                    label: getFrequencyLabel(weeklyDays, timesPerMonth)
                });
            });

            if (sideFreqs.length === 0) {
                return {daysPerWeek: 0, timesPerMonth: 0, label: "Less than monthly"};
            }

            var firstLabel = sideFreqs[0].label;
            var allSame = sideFreqs.every(function(freq) {
                return freq.label === firstLabel;
            });

            var maxFreq = sideFreqs[0];
            sideFreqs.slice(1).forEach(function(freq) {
                if (freq.daysPerWeek > maxFreq.daysPerWeek) {
                    maxFreq = freq;
                    return;
                }
                if (freq.daysPerWeek === maxFreq.daysPerWeek &&
                    freq.timesPerMonth > maxFreq.timesPerMonth) {
                    maxFreq = freq;
                }
            });

            return {
                daysPerWeek: maxFreq.daysPerWeek,
                timesPerMonth: maxFreq.timesPerMonth,
                label: allSame ? firstLabel : "",
                perSide: allSame ? [] : sideFreqs
            };
        }

        var segments = '
FROM source_meta;

-- Generate GeoJSON features for each street segment (both sides combined)
WITH schedule_base AS (
    SELECT
        CNN,
        Corridor,
        Limits,
        WeekDay,
        FromHour,
        ToHour,
        Week1,
        Week2,
        Week3,
        Week4,
        Week5,
        Holidays,
        Line,
        CASE CNNRightLeft WHEN 'L' THEN 'Left' WHEN 'R' THEN 'Right' ELSE CNNRightLeft END ||
            CASE WHEN BlockSide IS NOT NULL THEN ' (' || BlockSide || ')' ELSE '' END AS side_label,
        CASE WeekDay
            WHEN 'Mon' THEN 1
            WHEN 'Tues' THEN 2
            WHEN 'Wed' THEN 4
            WHEN 'Thu' THEN 8
            WHEN 'Fri' THEN 16
            WHEN 'Sat' THEN 32
            WHEN 'Sun' THEN 64
            ELSE 0
        END AS day_bit,
        CASE WeekDay
            WHEN 'Mon' THEN 1
            WHEN 'Tues' THEN 2
            WHEN 'Wed' THEN 3
            WHEN 'Thu' THEN 4
            WHEN 'Fri' THEN 5
            WHEN 'Sat' THEN 6
            WHEN 'Sun' THEN 7
            WHEN 'Holiday' THEN 8
            ELSE 9
        END AS weekday_order,
        CASE WeekDay
            WHEN 'Tues' THEN 'Tue'
            ELSE WeekDay
        END AS weekday_short
    FROM street_sweeping
    WHERE Line IS NOT NULL
),
side_frequency AS (
    SELECT
        CNN,
        Corridor,
        Limits,
        MAX(COUNT(DISTINCT CASE WHEN WeekDay NOT IN ('Holiday') THEN WeekDay END)) OVER (PARTITION BY CNN) as days_per_week,
        SUM(Week1 + Week2 + Week3 + Week4 + Week5) as times_per_month,
        MIN(FromHour) as min_from_hour,
        MAX(ToHour) as max_to_hour,
        ANY_VALUE(Line) as Line
    FROM schedule_base
    GROUP BY CNN, Corridor, Limits
),
days_by_cnn AS (
    SELECT
        CNN,
        Corridor,
        Limits,
        string_agg(WeekDay, ', ' ORDER BY weekday_order) AS all_days
    FROM (
        SELECT DISTINCT
            CNN,
            Corridor,
            Limits,
            WeekDay,
            weekday_order
        FROM schedule_base
    ) distinct_days
    GROUP BY CNN, Corridor, Limits
),
schedule_grouped AS (
    SELECT
        CNN,
        Corridor,
        Limits,
        side_label,
        FromHour,
        ToHour,
        Week1,
        Week2,
        Week3,
        Week4,
        Week5,
        Holidays,
        bit_or(day_bit) AS days_mask,
        MAX(CASE WHEN WeekDay = 'Holiday' THEN 1 ELSE 0 END) AS has_holiday_day,
        CASE WHEN Week1 = 1 THEN 1 ELSE 0 END +
        CASE WHEN Week2 = 1 THEN 2 ELSE 0 END +
        CASE WHEN Week3 = 1 THEN 4 ELSE 0 END +
        CASE WHEN Week4 = 1 THEN 8 ELSE 0 END +
        CASE WHEN Week5 = 1 THEN 16 ELSE 0 END AS weeks_mask
    FROM schedule_base
    GROUP BY
        CNN,
        Corridor,
        Limits,
        side_label,
        FromHour,
        ToHour,
        Week1,
        Week2,
        Week3,
        Week4,
        Week5,
        Holidays
),
schedule_merged AS (
    SELECT
        CNN,
        Corridor,
        Limits,
        side_label,
        FromHour,
        ToHour,
        bit_or(weeks_mask) AS weeks_mask,
        MAX(Holidays) AS includes_holidays,
        days_mask,
        MAX(has_holiday_day) AS has_holiday_day
    FROM schedule_grouped
    GROUP BY
        CNN,
        Corridor,
        Limits,
        side_label,
        FromHour,
        ToHour,
        days_mask
),
schedule_details AS (
    SELECT
        CNN,
        Corridor,
        Limits,
        list(
            json_array(
                side_label,
                FromHour,
                ToHour,
                days_mask,
                weeks_mask,
                CASE WHEN includes_holidays = 1 THEN 1 ELSE 0 END,
                CASE WHEN has_holiday_day = 1 THEN 1 ELSE 0 END
            )
            ORDER BY side_label, FromHour, ToHour, days_mask
        ) AS schedule_details
    FROM schedule_merged
    GROUP BY CNN, Corridor, Limits
)
SELECT json_group_array(
    json_object(
        'type', 'Feature',
        'properties', json_object(
            'name', Corridor,
            'limits', Limits,
            'daysPerWeek', days_per_week,
            'timesPerMonth', times_per_month,
            'scheduleDetails', COALESCE(schedule_details, list_value())
        ),
        'geometry', json_object(
            'type', 'LineString',
            'coordinates', (
                SELECT json_group_array(
                    json_array(
                        CAST(split_part(coord, ' ', 1) AS DOUBLE),
                        CAST(split_part(coord, ' ', 2) AS DOUBLE)
                    )
                )
                FROM (
                    SELECT unnest(string_split(
                        regexp_replace(Line, 'LINESTRING \(|\)', '', 'g'),
                        ', '
                    )) as coord
                )
            )
        )
    )
) as feature
FROM side_frequency
LEFT JOIN days_by_cnn USING (CNN, Corridor, Limits)
LEFT JOIN schedule_details USING (CNN, Corridor, Limits);

-- JavaScript to render the map
SELECT '
        ;

        segments.forEach(function(feature) {
            if (!feature.geometry || !feature.geometry.coordinates || feature.geometry.coordinates.length === 0) {
                return;
            }

            var coords = feature.geometry.coordinates.map(function(c) {
                return [c[1], c[0]];
            });

            var scheduleDetails = decodeScheduleDetails(feature.properties.scheduleDetails || []);
            var freq = computeFrequency(scheduleDetails);
            var color = getColor(freq.daysPerWeek, freq.timesPerMonth);
            var freqHtml = "<b>Frequency:</b> ";
            if (freq.perSide && freq.perSide.length > 0) {
                freqHtml += "<ul style=\"margin:5px 0;padding-left:20px;\">";
                freq.perSide.forEach(function(sideFreq) {
                    freqHtml += "<li><b>" + sideFreq.side + ":</b> " + sideFreq.label + "</li>";
                });
                freqHtml += "</ul>";
            } else {
                freqHtml += freq.label;
            }

            // Build detailed schedule list
            var scheduleHtml = "";
            if (scheduleDetails.length > 0) {
                scheduleHtml = "<ul style=\"margin:5px 0;padding-left:20px;\">";
                scheduleDetails.forEach(function(sched) {
                    var scheduleText = formatScheduleText(sched);
                    if (scheduleText) {
                        scheduleHtml += "<li><b>" + sched.side + ":</b> " + scheduleText + "</li>";
                    }
                });
                scheduleHtml += "</ul>";
            }

            var popup = "<b>" + feature.properties.name + "</b><br>" +
                       "<b>Location:</b> " + feature.properties.limits + "<br>" +
                       freqHtml + "<br>" +
                       "<b>Schedule:</b>" + scheduleHtml;

            L.polyline(coords, {
                color: color,
                weight: 3,
                opacity: 0.7
            }).bindPopup(popup).addTo(map);
        });

        var legend = L.control({position: "bottomright"});
        legend.onAdd = function(map) {
            var div = L.DomUtil.create("div", "legend");
            div.innerHTML = "<h4>Street Sweeping Frequency</h4>" +
                "<div class=\"legend-item\"><span class=\"legend-color\" style=\"background:#67001f\"></span>Daily (7x/week)</div>" +
                "<div class=\"legend-item\"><span class=\"legend-color\" style=\"background:#d73027\"></span>5-6x per week</div>" +
                "<div class=\"legend-item\"><span class=\"legend-color\" style=\"background:#fc8d59\"></span>3-4x per week</div>" +
                "<div class=\"legend-item\"><span class=\"legend-color\" style=\"background:#fee090\"></span>2x per week</div>" +
                "<div class=\"legend-item\"><span class=\"legend-color\" style=\"background:#91bfdb\"></span>Weekly (4-5x/month)</div>" +
                "<div class=\"legend-item\"><span class=\"legend-color\" style=\"background:#abd9e9\"></span>3x per month</div>" +
                "<div class=\"legend-item\"><span class=\"legend-color\" style=\"background:#e0f3f8\"></span>Biweekly (2x/month)</div>" +
                "<div class=\"legend-item\"><span class=\"legend-color\" style=\"background:#ffffbf\"></span>Monthly</div>";
            return div;
        };
        legend.addTo(map);
    </script>
</body>
</html>';
