
#include <Arduino.h>
#include <array>

constexpr std::array<int, 3> rows{{32, 33, 25}};
constexpr std::array<int, 4> columns{{26, 27, 14, 12}};
constexpr int indicator{13};

void setup()
{
	Serial.begin(115200);
	pinMode(indicator, OUTPUT);
	for (auto &row : rows)
	{
		pinMode(row, OUTPUT);
		digitalWrite(row, HIGH);
	}
	for (auto &column : columns)
	{
		pinMode(column, INPUT_PULLUP);
	}
}

void loop()
{
	for (int i = 0; i < rows.size(); i++)
	{
		digitalWrite(rows[i], LOW);
		for (int j = 0; j < columns.size(); j++)
		{
			digitalWrite(indicator, LOW);
			if (digitalRead(columns[j]) == LOW)
			{
				digitalWrite(indicator, HIGH);
				Serial.write(i * 4 + j);
				delay(200);
			}
		}
		digitalWrite(rows[i], HIGH);
	}
}