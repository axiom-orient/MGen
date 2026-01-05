import SwiftUI

/// Maintenance View.
/// iOS equivalent of Android MaintenanceScreen.
///
/// Displayed when the server is under maintenance.
/// Blocks app usage until maintenance is complete.
/// Priority: Higher than ForceUpdate.
struct MaintenanceView: View {
    let message: String
    var onRefresh: (() -> Void)? = nil

    var body: some View {
        VStack(spacing: 24) {
            Spacer()

            // Maintenance Icon
            Image(systemName: "wrench.and.screwdriver.fill")
                .font(.system(size: 60))
                .foregroundStyle(.secondary)

            Text("서버 점검 중")
                .font(.title2)
                .bold()

            Text(message.isEmpty ? "잠시 후 다시 시도해주세요." : message)
                .font(.subheadline)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal, 32)

            Spacer()

            if let onRefresh {
                Button(action: onRefresh) {
                    Text("다시 시도")
                        .bold()
                        .frame(maxWidth: .infinity)
                        .padding()
                        .background(Color.secondary.opacity(0.2))
                        .foregroundStyle(.primary)
                        .cornerRadius(12)
                }
                .padding(.horizontal, 40)
                .padding(.bottom, 40)
            }
        }
    }
}

#Preview {
    MaintenanceView(
        message: "서버 점검 중입니다. 잠시 후 다시 시도해주세요.",
        onRefresh: {}
    )
}
