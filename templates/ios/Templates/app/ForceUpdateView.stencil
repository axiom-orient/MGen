import SwiftUI

/// Force Update View.
/// iOS equivalent of Android ForceUpdateScreen.
///
/// Displayed when the app version is older than the required minimum version.
/// Blocks app usage until user updates.
struct ForceUpdateView: View {
    let requiredVersion: String
    let storeURL: URL

    var body: some View {
        VStack(spacing: 24) {
            Spacer()

            // Update Icon
            Image(systemName: "arrow.clockwise.circle.fill")
                .font(.system(size: 60))
                .foregroundStyle(.orange)

            Text("업데이트가 필요합니다")
                .font(.title2)
                .bold()

            Text("최신 버전(\(requiredVersion))으로 업데이트 후 이용해주세요.")
                .font(.subheadline)
                .foregroundStyle(.secondary)
                .multilineTextAlignment(.center)

            Spacer()

            Link(destination: storeURL) {
                Text("업데이트 하러 가기")
                    .bold()
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color.orange)
                    .foregroundStyle(.white)
                    .cornerRadius(12)
            }
            .padding(.horizontal, 40)
            .padding(.bottom, 40)
        }
    }
}

#Preview {
    ForceUpdateView(
        requiredVersion: "2.0.0",
        storeURL: URL(string: "https://apps.apple.com")!
    )
}
